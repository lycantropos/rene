use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ffi::c_long;
use std::ops::Deref;

use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyIndexError, PyOverflowError, PyTypeError, PyValueError,
    PyZeroDivisionError,
};
use pyo3::prelude::{
    pyclass, pymethods, pymodule, PyModule, PyResult, Python,
};
use pyo3::type_object::PyTypeInfo;
use pyo3::types::{
    PyFloat, PyFrozenSet, PyLong, PySequence, PySlice, PyString, PyTuple,
    PyType,
};
use pyo3::{
    ffi, intern, AsPyPointer, FromPyObject, FromPyPointer, IntoPy, Py, PyAny,
    PyErr, PyObject, PyRef, ToPyObject,
};
use rithm::{big_int, fraction};
use traiter::numbers::{Endianness, FromBytes, ToBytes, Zero};

use crate::bentley_ottmann::{is_contour_valid, is_multisegment_valid};
use crate::bounded::Bounded;
use crate::constants::{
    MIN_CONTOUR_VERTICES_COUNT, MIN_MULTIPOLYGON_POLYGONS_COUNT,
    MIN_MULTISEGMENT_SEGMENTS_COUNT,
};
use crate::locatable::{Locatable, Location};
use crate::operations::{permute, to_arg_min};
use crate::oriented::{Orientation, Oriented};
use crate::relatable::{Relatable, Relation};
use crate::seidel::Trapezoidation;
use crate::slice_sequence::SliceSequence;
use crate::traits::{
    Difference, Elemental, Intersection, Iterable, Lengthsome, Multipolygonal,
    Multisegmental, Multivertexal, Polygonal, Segmental, Sequence,
    SymmetricDifference, Union,
};
use crate::triangulation::{
    BoundaryEndpoints, ConstrainedDelaunayTriangulation, DelaunayTriangulation,
};

mod bentley_ottmann;
pub mod bounded;
mod clipping;
mod constants;
mod contracts;
pub mod geometries;
mod iteration;
pub mod locatable;
mod operations;
pub mod oriented;
pub mod relatable;
mod relating;
mod seidel;
mod slice_sequence;
mod sweeping;
pub mod traits;
mod triangulation;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const DIGIT_BITNESS: usize = (Digit::BITS - 1) as usize;
const _: () =
    assert!(big_int::is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());

type BigInt = big_int::BigInt<Digit, DIGIT_BITNESS>;
type Fraction = fraction::Fraction<BigInt>;
type Empty = geometries::Empty;
type ExactBox = bounded::Box<Fraction>;
type ExactConstrainedDelaunayTriangulation =
    ConstrainedDelaunayTriangulation<ExactPoint>;
type ExactContour = geometries::Contour<Fraction>;
type ExactDelaunayTriangulation = DelaunayTriangulation<ExactPoint>;
type ExactMultipolygon = geometries::Multipolygon<Fraction>;
type ExactMultisegment = geometries::Multisegment<Fraction>;
type ExactPoint = geometries::Point<Fraction>;
type ExactPolygon = geometries::Polygon<Fraction>;
type ExactSegment = geometries::Segment<Fraction>;
type ExactTrapezoidation = Trapezoidation<ExactPoint>;

mod reference {
    use pyo3::{AsPyPointer, PyClass, PyObject, PyRef};
    use std::ops::Deref;

    #[derive(Clone)]
    pub(super) struct Reference<T> {
        _python_value: PyObject,
        rust_ptr: *const T,
    }

    unsafe impl<T> Send for Reference<T> {}

    impl<T> Deref for Reference<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.rust_ptr }
        }
    }

    impl<T> Reference<T> {
        pub(super) fn from_py_ref(value: PyRef<T>) -> Self
        where
            T: PyClass,
        {
            Reference {
                _python_value: unsafe {
                    PyObject::from_borrowed_ptr(value.py(), value.as_ptr())
                },
                rust_ptr: value.deref() as *const T,
            }
        }
    }
}

use reference::Reference;

trait Count<T> {
    fn count(&self, value: &T) -> usize;
}

impl<'a, T: PartialEq> Count<T> for SliceSequence<'a, T> {
    fn count(&self, value: &T) -> usize {
        self.iter()
            .filter(|&candidate| candidate.eq(&value))
            .count()
    }
}

trait Position<T> {
    fn position(
        &self,
        value: &T,
        start: Option<usize>,
        stop: Option<usize>,
    ) -> Option<usize>;
}

impl<'a, T: PartialEq> Position<T> for SliceSequence<'a, T> {
    fn position(
        &self,
        value: &T,
        start: Option<usize>,
        stop: Option<usize>,
    ) -> Option<usize> {
        let start = start.unwrap_or(0usize);
        let stop = stop.unwrap_or_else(|| self.len());
        self.iter()
            .skip(start)
            .take(stop.saturating_sub(start))
            .position(|candidate| candidate.eq(&value))
            .map(|offset| start + offset)
    }
}

enum GenericIterator<I> {
    Forward(I),
    Backward(std::iter::Rev<I>),
}

impl<I: DoubleEndedIterator> Iterator for GenericIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Forward(iterator) => iterator.next(),
            Self::Backward(iterator) => iterator.next(),
        }
    }
}

impl<I: DoubleEndedIterator> ExactSizeIterator for GenericIterator<I> {
    fn len(&self) -> usize {
        todo!()
    }
}

impl<I: DoubleEndedIterator> GenericIterator<I>
where
    I::Item: PartialEq,
{
    fn contains(&mut self, value: I::Item) -> bool {
        self.any(|candidate| candidate == value)
    }
}

type PyExactMultisegmentReference = Reference<PyExactMultisegment>;

#[pyclass(name = "_MultisegmentSegments", module = "rene.exact", sequence)]
struct PyExactMultisegmentSegments {
    multisegment: PyExactMultisegmentReference,
    start: isize,
    stop: isize,
    step: isize,
}

impl PyExactMultisegmentSegments {
    fn iter(
        &self,
    ) -> GenericIterator<
        std::iter::Take<
            std::iter::StepBy<std::iter::Skip<std::slice::Iter<ExactSegment>>>,
        >,
    > {
        if self.step > 0 {
            GenericIterator::Forward(
                (&self.multisegment.deref().0)
                    .segments()
                    .into_iter()
                    .skip(self.start as usize)
                    .step_by(self.step as usize)
                    .take(self.len()),
            )
        } else {
            let segments_count = self.len();
            GenericIterator::Backward(
                (&self.multisegment.deref().0)
                    .segments()
                    .into_iter()
                    .skip(
                        (self.start
                            + ((segments_count as isize) - 1) * self.step)
                            as usize,
                    )
                    .step_by((-self.step) as usize)
                    .take(segments_count)
                    .rev(),
            )
        }
    }

    fn len(&self) -> usize {
        (if self.step > 0 && self.start < self.stop {
            1 + (self.stop - self.start - 1) / self.step
        } else if self.step < 0 && self.start > self.stop {
            1 + (self.start - self.stop - 1) / (-self.step)
        } else {
            0
        }) as usize
    }
}

#[pymethods]
impl PyExactMultisegmentSegments {
    #[pyo3(signature = (segment, /))]
    fn count(&self, segment: &PyExactSegment) -> usize {
        self.iter()
            .filter(|&candidate| candidate == &segment.0)
            .count()
    }

    #[pyo3(signature = (segment, start=None, stop=None, /))]
    fn index(
        &self,
        segment: &PyExactSegment,
        start: Option<&PyLong>,
        stop: Option<&PyLong>,
        py: Python,
    ) -> PyResult<usize> {
        match {
            let elements_count = self.len();
            let start = normalize_index_start(start, elements_count);
            let stop = normalize_index_stop(stop, elements_count);
            self.iter()
            .skip(start)
            .take(stop.saturating_sub(start))
            .position(|candidate| candidate.eq(&segment.0))
            .map(|offset| start + offset)
        }
        {
            Some(result) => Ok(result),
            None => Err(PyValueError::new_err(format!(
                "{} is not found among multisegment segments with indices from range({}, {}).",
                segment.__repr__()?,
                match start {
                    Some(start) => start.repr()?,
                    None => intern!(py, "0"),
                },
                match stop {
                    Some(stop) => stop.repr()?,
                    None =>
                        PyString::new(py, &self.len().to_string()),
                }
            ))),
        }
    }

    fn __contains__(&self, value: &PyExactSegment) -> bool {
        self.iter().contains(&value.0)
    }

    fn __getitem__(&self, item: &PyAny, py: Python) -> PyResult<PyObject> {
        if item.is_instance(PySlice::type_object(py))? {
            let (start, stop, step) = to_next_slice_indices(
                self.start,
                self.step,
                self.len(),
                item.extract::<&PySlice>()?,
            )?;
            Ok(Self {
                multisegment: self.multisegment.clone(),
                start,
                stop,
                step,
            }
            .into_py(py))
        } else {
            let maybe_index = unsafe { ffi::PyNumber_Index(item.as_ptr()) };
            if maybe_index.is_null() {
                Err(PyErr::fetch(item.py()))
            } else {
                let index = py_long_to_valid_index(
                    unsafe { PyLong::from_owned_ptr(item.py(), maybe_index) },
                    self.len(),
                )?;
                Ok(unsafe { self.iter().nth(index).unwrap_unchecked() }
                    .clone()
                    .into_py(py))
            }
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyTuple::new(py, self.iter().collect::<Vec<_>>()).hash()
    }

    fn __len__(&self) -> usize {
        self.len()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(Self::type_object(py))? {
            let other = other.extract::<PyRef<Self>>()?;
            match op {
                CompareOp::Eq => Ok(self.iter().eq(other.iter()).into_py(py)),
                CompareOp::Ne => Ok(self.iter().ne(other.iter()).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

type PyExactContourReference = Reference<PyExactContour>;

#[pyclass(name = "_ContourSegments", module = "rene.exact", sequence)]
struct PyExactContourSegments(PyExactContourReference);

impl PyExactContourSegments {
    fn get_segments(&self) -> SliceSequence<ExactSegment> {
        (&self.0.deref().0).segments()
    }
}

fn try_multiply_isizes(first: isize, second: isize) -> PyResult<isize> {
    if let (result, false) = first.overflowing_mul(second) {
        Ok(result)
    } else {
        Err(PyOverflowError::new_err(format!(
            "Multiplication of {} & {} is out of range({}, {}).",
            first,
            second,
            isize::MIN,
            (isize::MAX as usize) + 1,
        )))
    }
}

fn try_sum_isizes(first: isize, second: isize) -> PyResult<isize> {
    if let (result, false) = first.overflowing_add(second) {
        Ok(result)
    } else {
        Err(PyOverflowError::new_err(format!(
            "Addition of {} & {} is out of range({}, {}).",
            first,
            second,
            isize::MIN,
            (isize::MAX as usize) + 1,
        )))
    }
}

fn normalize_index_start(
    start: Option<&PyLong>,
    elements_count: usize,
) -> usize {
    start
        .map(|value| {
            value
                .extract::<isize>()
                .map(|value| {
                    (if value < 0 {
                        (value + (elements_count as isize)).max(0)
                    } else {
                        value
                    }) as usize
                })
                .unwrap_or(elements_count)
        })
        .unwrap_or(0usize)
}

fn normalize_index_stop(
    start: Option<&PyLong>,
    elements_count: usize,
) -> usize {
    start
        .map(|value| {
            value
                .extract::<isize>()
                .map(|value| {
                    (if value < 0 {
                        (value + (elements_count as isize)).max(0)
                    } else {
                        value
                    }) as usize
                })
                .unwrap_or(0)
        })
        .unwrap_or(elements_count)
}

fn to_next_slice_indices(
    start: isize,
    step: isize,
    length: usize,
    slice: &PySlice,
) -> Result<(isize, isize, isize), PyErr> {
    let indices = slice.indices(length as c_long)?;
    let result_step = try_multiply_isizes(step, indices.step)?;
    let result_start =
        try_sum_isizes(start, try_multiply_isizes(step, indices.start)?)?;
    let result_stop =
        try_sum_isizes(start, try_multiply_isizes(step, indices.stop)?)?;
    Ok((result_start, result_stop, result_step))
}

#[pymethods]
impl PyExactContourSegments {
    #[pyo3(signature = (segment, /))]
    fn count(&self, segment: &PyExactSegment) -> usize {
        self.get_segments().count(&segment.0)
    }

    #[pyo3(signature = (segment, start=None, stop=None, /))]
    fn index(
        &self,
        segment: &PyExactSegment,
        start: Option<&PyLong>,
        stop: Option<&PyLong>,
        py: Python,
    ) -> PyResult<usize> {
        let segments = self.get_segments();
        match segments.position(
            &segment.0,
            start.map(|value| value.extract::<usize>().unwrap_or(0usize)),
            stop.map(|value| value.extract::<usize>().unwrap_or(0usize))
            )
        {
            Some(result) => Ok(result),
            None => Err(PyValueError::new_err(format!(
                "{} is not found among contour segments with indices from range({}, {}).",
                segment.__repr__()?,
                match start {
                    Some(start) => start.repr()?,
                    None => intern!(py, "0"),
                },
                match stop {
                    Some(stop) => stop.repr()?,
                    None =>
                        PyString::new(py, &segments.len().to_string()),
                }
            ))),
        }
    }

    fn __contains__(&self, value: &PyExactSegment) -> bool {
        self.get_segments().contains(&value.0)
    }

    fn __getitem__(&self, item: &PyAny) -> PyResult<PyExactSegment> {
        sequence_get_item(&self.get_segments(), item).map(PyExactSegment)
    }

    fn __len__(&self) -> usize {
        self.get_segments().len()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContourSegments::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContourSegments>>()?;
            match op {
                CompareOp::Eq => Ok(self
                    .get_segments()
                    .eq(&other.get_segments())
                    .into_py(py)),
                CompareOp::Ne => Ok(self
                    .get_segments()
                    .ne(&other.get_segments())
                    .into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyclass(name = "_ContourVertices", module = "rene.exact", sequence)]
struct PyExactContourVertices(PyExactContourReference);

impl PyExactContourVertices {
    fn get_vertices(&self) -> SliceSequence<ExactPoint> {
        (&self.0.deref().0).vertices()
    }
}

#[pymethods]
impl PyExactContourVertices {
    #[pyo3(signature = (point, /))]
    fn count(&self, point: &PyExactPoint) -> usize {
        self.get_vertices().count(&point.0)
    }

    #[pyo3(signature = (point, start=None, stop=None, /))]
    fn index(
        &self,
        point: &PyExactPoint,
        start: Option<&PyLong>,
        stop: Option<&PyLong>,
        py: Python,
    ) -> PyResult<usize> {
        let vertices = self.get_vertices();
        match vertices.position(
                &point.0,
                start.map(|value| value.extract::<usize>().unwrap_or(0usize)),
                stop.map(|value| value.extract::<usize>().unwrap_or(0usize))
            )
        {
            Some(result) => Ok(result),
            None => Err(PyValueError::new_err(format!(
                "{} is not found among contour vertices with indices from range({}, {}).",
                point.__repr__()?,
                match start {
                    Some(start) => start.repr()?,
                    None => intern!(py, "0"),
                },
                match stop {
                    Some(stop) => stop.repr()?,
                    None =>
                        PyString::new(py, &vertices.len().to_string()),
                }
            ))),
        }
    }

    fn __contains__(&self, value: &PyExactPoint) -> bool {
        self.get_vertices().contains(&value.0)
    }

    fn __getitem__(&self, item: &PyAny) -> PyResult<PyExactPoint> {
        sequence_get_item(&self.get_vertices(), item).map(PyExactPoint)
    }

    fn __len__(&self) -> usize {
        self.get_vertices().len()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContourVertices::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContourVertices>>()?;
            match op {
                CompareOp::Eq => Ok(self
                    .get_vertices()
                    .eq(&other.get_vertices())
                    .into_py(py)),
                CompareOp::Ne => Ok(self
                    .get_vertices()
                    .ne(&other.get_vertices())
                    .into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

type PyExactPolygonReference = Reference<PyExactPolygon>;

#[pyclass(name = "_PolygonHoles", module = "rene.exact", sequence)]
struct PyExactPolygonHoles(PyExactPolygonReference);

impl PyExactPolygonHoles {
    fn get_holes(&self) -> SliceSequence<ExactContour> {
        (&self.0.deref().0).holes()
    }
}

#[pymethods]
impl PyExactPolygonHoles {
    #[pyo3(signature = (contour, /))]
    fn count(&self, contour: &PyExactContour) -> usize {
        self.get_holes().count(&contour.0)
    }

    #[pyo3(signature = (contour, start=None, stop=None, /))]
    fn index(
        &self,
        contour: &PyExactContour,
        start: Option<&PyLong>,
        stop: Option<&PyLong>,
        py: Python,
    ) -> PyResult<usize> {
        let holes = self.get_holes();
        match holes.position(
            &contour.0,
            start.map(|value| value.extract::<usize>().unwrap_or(0usize)),
            stop.map(|value| value.extract::<usize>().unwrap_or(0usize))
            )
        {
            Some(result) => Ok(result),
            None => Err(PyValueError::new_err(format!(
                "{} is not found among polygon holes with indices from range({}, {}).",
                contour.__repr__()?,
                match start {
                    Some(start) => start.repr()?,
                    None => intern!(py, "0"),
                },
                match stop {
                    Some(stop) => stop.repr()?,
                    None =>
                        PyString::new(py, &holes.len().to_string()),
                }
            ))),
        }
    }

    fn __contains__(&self, value: &PyExactContour) -> bool {
        self.get_holes().contains(&value.0)
    }

    fn __getitem__(&self, item: &PyAny) -> PyResult<PyExactContour> {
        sequence_get_item(&self.get_holes(), item).map(PyExactContour)
    }

    fn __len__(&self) -> usize {
        self.get_holes().len()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactPolygonHoles::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygonHoles>>()?;
            match op {
                CompareOp::Eq => {
                    Ok(self.get_holes().eq(&other.get_holes()).into_py(py))
                }
                CompareOp::Ne => {
                    Ok(self.get_holes().ne(&other.get_holes()).into_py(py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

type PyExactMultipolygonReference = Reference<PyExactMultipolygon>;

#[pyclass(name = "_MultipolygonPolygons", module = "rene.exact", sequence)]
struct PyExactMultipolygonPolygons(PyExactMultipolygonReference);

impl PyExactMultipolygonPolygons {
    fn get_polygons(&self) -> SliceSequence<ExactPolygon> {
        (&self.0.deref().0).polygons()
    }
}

#[pymethods]
impl PyExactMultipolygonPolygons {
    #[pyo3(signature = (polygon, /))]
    fn count(&self, polygon: &PyExactPolygon) -> usize {
        self.get_polygons().count(&polygon.0)
    }

    #[pyo3(signature = (polygon, start=None, stop=None, /))]
    fn index(
        &self,
        polygon: &PyExactPolygon,
        start: Option<&PyLong>,
        stop: Option<&PyLong>,
        py: Python,
    ) -> PyResult<usize> {
        let polygons = self.get_polygons();
        match polygons.position(
            &polygon.0,
            start.map(|value| value.extract::<usize>().unwrap_or(0usize)),
            stop.map(|value| value.extract::<usize>().unwrap_or(0usize))
            )
        {
            Some(result) => Ok(result),
            None => Err(PyValueError::new_err(format!(
                "{} is not found among multipolygon polygons with indices from range({}, {}).",
                polygon.__repr__(py)?,
                match start {
                    Some(start) => start.repr()?,
                    None => intern!(py, "0"),
                },
                match stop {
                    Some(stop) => stop.repr()?,
                    None =>
                        PyString::new(py, &polygons.len().to_string()),
                }
            ))),
        }
    }

    fn __contains__(&self, value: &PyExactPolygon) -> bool {
        self.get_polygons().contains(&value.0)
    }

    fn __getitem__(&self, item: &PyAny) -> PyResult<PyExactPolygon> {
        sequence_get_item(&self.get_polygons(), item).map(PyExactPolygon)
    }

    fn __len__(&self) -> usize {
        self.get_polygons().len()
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactMultipolygonPolygons::type_object(py))? {
            let other =
                other.extract::<PyRef<PyExactMultipolygonPolygons>>()?;
            match op {
                CompareOp::Eq => Ok(self
                    .get_polygons()
                    .eq(&other.get_polygons())
                    .into_py(py)),
                CompareOp::Ne => Ok(self
                    .get_polygons()
                    .ne(&other.get_polygons())
                    .into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

impl IntoPy<PyObject> for ExactBox {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactBox(self).into_py(py)
    }
}

impl IntoPy<PyObject> for ExactContour {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactContour(self).into_py(py)
    }
}

impl IntoPy<PyObject> for ExactMultipolygon {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactMultipolygon(self).into_py(py)
    }
}

impl IntoPy<PyObject> for ExactPoint {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactPoint(self).into_py(py)
    }
}

impl IntoPy<PyObject> for ExactPolygon {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactPolygon(self).into_py(py)
    }
}

impl IntoPy<PyObject> for ExactSegment {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactSegment(self).into_py(py)
    }
}

impl IntoPy<PyObject> for Relation {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyRelation(self).into_py(py)
    }
}

impl ToPyObject for Relation {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.into_py(py)
    }
}

impl ToPyObject for ExactContour {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

impl ToPyObject for ExactPoint {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

impl ToPyObject for ExactPolygon {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

impl ToPyObject for ExactSegment {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

impl From<PyExactContour> for ExactContour {
    fn from(value: PyExactContour) -> Self {
        value.0
    }
}

impl From<PyExactPoint> for ExactPoint {
    fn from(value: PyExactPoint) -> Self {
        value.0
    }
}

impl From<PyExactPolygon> for ExactPolygon {
    fn from(value: PyExactPolygon) -> Self {
        value.0
    }
}

impl From<PyExactSegment> for ExactSegment {
    fn from(value: PyExactSegment) -> Self {
        value.0
    }
}

#[pyclass(name = "Location", module = "rene")]
struct PyLocation(Location);

#[pymethods]
impl PyLocation {
    #[classattr]
    const BOUNDARY: PyLocation = PyLocation(Location::Boundary);

    #[classattr]
    const EXTERIOR: PyLocation = PyLocation(Location::Exterior);

    #[classattr]
    const INTERIOR: PyLocation = PyLocation(Location::Interior);

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Location::Boundary => "BOUNDARY",
                Location::Exterior => "EXTERIOR",
                Location::Interior => "INTERIOR",
            }
        )
    }
}

#[pyclass(name = "Orientation", module = "rene")]
struct PyOrientation(Orientation);

#[pymethods]
impl PyOrientation {
    #[classattr]
    const CLOCKWISE: PyOrientation = PyOrientation(Orientation::Clockwise);

    #[classattr]
    const COLLINEAR: PyOrientation = PyOrientation(Orientation::Collinear);

    #[classattr]
    const COUNTERCLOCKWISE: PyOrientation =
        PyOrientation(Orientation::Counterclockwise);

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Orientation::Clockwise => "CLOCKWISE",
                Orientation::Collinear => "COLLINEAR",
                Orientation::Counterclockwise => "COUNTERCLOCKWISE",
            }
        )
    }
}

#[pyclass(name = "Relation", module = "rene")]
struct PyRelation(Relation);

#[pymethods]
impl PyRelation {
    #[classattr]
    const COMPONENT: PyRelation = PyRelation(Relation::Component);

    #[classattr]
    const COMPOSITE: PyRelation = PyRelation(Relation::Composite);

    #[classattr]
    const COVER: PyRelation = PyRelation(Relation::Cover);

    #[classattr]
    const CROSS: PyRelation = PyRelation(Relation::Cross);

    #[classattr]
    const DISJOINT: PyRelation = PyRelation(Relation::Disjoint);

    #[classattr]
    const ENCLOSED: PyRelation = PyRelation(Relation::Enclosed);

    #[classattr]
    const ENCLOSES: PyRelation = PyRelation(Relation::Encloses);

    #[classattr]
    const EQUAL: PyRelation = PyRelation(Relation::Equal);

    #[classattr]
    const OVERLAP: PyRelation = PyRelation(Relation::Overlap);

    #[classattr]
    const TOUCH: PyRelation = PyRelation(Relation::Touch);

    #[classattr]
    const WITHIN: PyRelation = PyRelation(Relation::Within);

    #[getter]
    fn complement<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        PyRelation::type_object(py).getattr(match self.0 {
            Relation::Component => intern!(py, "COMPOSITE"),
            Relation::Composite => intern!(py, "COMPONENT"),
            Relation::Cover => intern!(py, "WITHIN"),
            Relation::Cross => intern!(py, "CROSS"),
            Relation::Disjoint => intern!(py, "DISJOINT"),
            Relation::Enclosed => intern!(py, "ENCLOSES"),
            Relation::Encloses => intern!(py, "ENCLOSED"),
            Relation::Equal => intern!(py, "EQUAL"),
            Relation::Overlap => intern!(py, "OVERLAP"),
            Relation::Touch => intern!(py, "TOUCH"),
            Relation::Within => intern!(py, "COVER"),
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "{}.{}",
            Self::NAME,
            match self.0 {
                Relation::Component => "COMPONENT",
                Relation::Composite => "COMPOSITE",
                Relation::Cover => "COVER",
                Relation::Cross => "CROSS",
                Relation::Disjoint => "DISJOINT",
                Relation::Enclosed => "ENCLOSED",
                Relation::Encloses => "ENCLOSES",
                Relation::Equal => "EQUAL",
                Relation::Overlap => "OVERLAP",
                Relation::Touch => "TOUCH",
                Relation::Within => "WITHIN",
            }
        )
    }
}

#[pyclass(name = "Box", module = "rene.exact")]
#[derive(Clone)]
struct PyExactBox(ExactBox);

#[pyclass(name = "ConstrainedDelaunayTriangulation", module = "rene.exact")]
#[derive(Clone)]
struct PyExactConstrainedDelaunayTriangulation(
    ExactConstrainedDelaunayTriangulation,
);

#[pyclass(name = "Contour", module = "rene.exact")]
#[derive(Clone)]
struct PyExactContour(ExactContour);

#[pyclass(name = "DelaunayTriangulation", module = "rene.exact")]
#[derive(Clone)]
struct PyExactDelaunayTriangulation(ExactDelaunayTriangulation);

#[pyclass(name = "Empty", module = "rene.exact")]
#[derive(Clone)]
struct PyExactEmpty(Empty);

#[pyclass(name = "Multipolygon", module = "rene.exact")]
#[derive(Clone)]
struct PyExactMultipolygon(ExactMultipolygon);

#[pyclass(name = "Multisegment", module = "rene.exact")]
#[derive(Clone)]
struct PyExactMultisegment(ExactMultisegment);

#[pyclass(name = "Point", module = "rene.exact")]
#[derive(Clone)]
struct PyExactPoint(ExactPoint);

#[pyclass(name = "Polygon", module = "rene.exact")]
#[derive(Clone)]
struct PyExactPolygon(ExactPolygon);

#[pyclass(name = "Segment", module = "rene.exact")]
#[derive(Clone)]
struct PyExactSegment(ExactSegment);

#[pyclass(name = "Trapezoidation", module = "rene.exact")]
#[derive(Clone)]
struct PyExactTrapezoidation(ExactTrapezoidation);

#[pymethods]
impl PyExactBox {
    #[new]
    #[pyo3(signature = (min_x, max_x, min_y, max_y, /))]
    fn new(
        min_x: &PyAny,
        max_x: &PyAny,
        min_y: &PyAny,
        max_y: &PyAny,
    ) -> PyResult<Self> {
        Ok(PyExactBox(ExactBox::new(
            try_scalar_to_fraction(min_x)?,
            try_scalar_to_fraction(max_x)?,
            try_scalar_to_fraction(min_y)?,
            try_scalar_to_fraction(max_y)?,
        )))
    }

    #[getter]
    fn max_x(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction(self.0.get_max_x())
    }

    #[getter]
    fn max_y(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction(self.0.get_max_y())
    }

    #[getter]
    fn min_x(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction(self.0.get_min_x())
    }

    #[getter]
    fn min_y(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction(self.0.get_min_y())
    }

    #[pyo3(signature = (other, /))]
    fn covers(&self, other: &Self) -> bool {
        self.0.covers(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn disjoint_with(&self, other: &Self) -> bool {
        self.0.disjoint_with(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn enclosed_by(&self, other: &Self) -> bool {
        self.0.enclosed_by(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn encloses(&self, other: &Self) -> bool {
        self.0.encloses(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn equals_to(&self, other: &Self) -> bool {
        self.0.equals_to(&other.0)
    }

    fn is_valid(&self) -> bool {
        self.0.get_min_x() <= self.0.get_max_x()
            && self.0.get_min_y() <= self.0.get_max_y()
    }

    #[pyo3(signature = (other, /))]
    fn overlaps(&self, other: &Self) -> bool {
        self.0.overlaps(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn relate_to(&self, other: &Self) -> PyResult<&PyAny> {
        try_relation_to_py_relation(self.0.relate_to(&other.0))
    }

    #[pyo3(signature = (other, /))]
    fn touches(&self, other: &Self) -> bool {
        self.0.touches(&other.0)
    }

    #[pyo3(signature = (other, /))]
    fn within(&self, other: &Self) -> bool {
        self.0.within(&other.0)
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyTuple::new(
            py,
            [self.min_x()?, self.max_x()?, self.min_y()?, self.max_y()?],
        )
        .hash()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            Self::NAME,
            self.min_x()?.repr()?.extract::<String>()?,
            self.max_x()?.repr()?.extract::<String>()?,
            self.min_y()?.repr()?.extract::<String>()?,
            self.max_y()?.repr()?.extract::<String>()?,
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactBox::type_object(py))? {
            let other = other.extract::<PyRef<PyExactBox>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            Self::NAME,
            self.min_x()?.str()?.extract::<String>()?,
            self.max_x()?.str()?.extract::<String>()?,
            self.min_y()?.str()?.extract::<String>()?,
            self.max_y()?.str()?.extract::<String>()?,
        ))
    }
}

#[pymethods]
impl PyExactConstrainedDelaunayTriangulation {
    #[classmethod]
    #[pyo3(signature = (polygon, /))]
    fn from_polygon(_: &PyType, polygon: &PyExactPolygon) -> Self {
        PyExactConstrainedDelaunayTriangulation(
            ExactConstrainedDelaunayTriangulation::from(&polygon.0),
        )
    }

    #[getter]
    fn border(&self) -> PyResult<PyExactContour> {
        try_vertices_to_py_exact_contour(
            self.0.get_boundary_points().into_iter().cloned().collect(),
        )
    }

    #[getter]
    fn triangles(&self) -> Vec<ExactContour> {
        self.0
            .to_triangles_vertices()
            .map(|(first, second, third)| {
                ExactContour::from([
                    first.clone(),
                    second.clone(),
                    third.clone(),
                ])
            })
            .collect()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }
}

#[pymethods]
impl PyExactContour {
    #[new]
    #[pyo3(signature = (vertices, /))]
    fn new(vertices: &PySequence) -> PyResult<Self> {
        try_vertices_to_py_exact_contour(extract_from_py_sequence::<
            PyExactPoint,
            ExactPoint,
        >(vertices)?)
    }

    #[getter]
    fn bounding_box(&self) -> ExactBox {
        (&self.0).to_bounding_box().cloned()
    }

    #[getter]
    fn segments(slf: PyRef<Self>) -> PyExactContourSegments {
        PyExactContourSegments(PyExactContourReference::from_py_ref(slf))
    }

    #[getter]
    fn segments_count(&self) -> usize {
        (&self.0).segments().len()
    }

    #[getter]
    fn vertices(slf: PyRef<Self>) -> PyExactContourVertices {
        PyExactContourVertices(PyExactContourReference::from_py_ref(slf))
    }

    #[getter]
    fn vertices_count(&self) -> usize {
        (&self.0).vertices().len()
    }

    #[getter]
    fn orientation(&self, py: Python) -> PyResult<&PyAny> {
        let orientation = (&self.0).to_orientation();
        let orientation_cls =
            unsafe { MAYBE_ORIENTATION_CLS.unwrap_unchecked() };
        match orientation {
            Orientation::Clockwise => {
                orientation_cls.getattr(intern!(py, "CLOCKWISE"))
            }
            Orientation::Collinear => {
                orientation_cls.getattr(intern!(py, "COLLINEAR"))
            }
            Orientation::Counterclockwise => {
                orientation_cls.getattr(intern!(py, "COUNTERCLOCKWISE"))
            }
        }
    }

    fn is_valid(&self) -> bool {
        is_contour_valid(&self.0)
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __and__(&self, other: &PyAny, py: Python) -> PyResult<PyObject> {
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            let segments = (&self.0).intersection(&other.0);
            match segments.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    segments.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultisegment(ExactMultisegment::new(segments))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            let segments = (&self.0).intersection(&other.0);
            match segments.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    segments.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultisegment(ExactMultisegment::new(segments))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        let mut vertices =
            (&self.0).vertices().into_iter().collect::<Vec<_>>();
        let min_vertex_index =
            unsafe { to_arg_min(&vertices).unwrap_unchecked() };
        vertices.rotate_left(min_vertex_index);
        if (&self.0).to_orientation() == Orientation::Clockwise {
            vertices[1..].reverse();
        }
        PyTuple::new(py, &vertices).hash()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}([{}])",
            Self::NAME,
            (&self.0)
                .vertices()
                .into_iter()
                .cloned()
                .map(|vertex| PyExactPoint(vertex).__repr__())
                .collect::<PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Contour([{}])",
            (&self.0)
                .vertices()
                .into_iter()
                .cloned()
                .map(|vertex| PyExactPoint(vertex).__str__())
                .collect::<PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }
}

#[pymethods]
impl PyExactDelaunayTriangulation {
    #[classmethod]
    #[pyo3(signature = (points, /))]
    fn from_points(_: &PyType, points: &PySequence) -> PyResult<Self> {
        Ok(PyExactDelaunayTriangulation(DelaunayTriangulation::from(
            extract_from_py_sequence::<PyExactPoint, ExactPoint>(points)?,
        )))
    }

    #[getter]
    fn border(&self) -> PyResult<PyExactContour> {
        try_vertices_to_py_exact_contour(
            self.0.get_boundary_points().into_iter().cloned().collect(),
        )
    }

    #[getter]
    fn triangles(&self) -> Vec<ExactContour> {
        self.0
            .iter_triangles_vertices()
            .map(|(first, second, third)| {
                ExactContour::from([
                    first.clone(),
                    second.clone(),
                    third.clone(),
                ])
            })
            .collect()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }
}

#[pymethods]
impl PyExactEmpty {
    #[new]
    fn new() -> Self {
        PyExactEmpty(Empty::new())
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint, py: Python) -> PyResult<&PyAny> {
        unsafe { MAYBE_LOCATION_CLS.unwrap_unchecked() }
            .getattr(intern!(py, "EXTERIOR"))
    }

    #[pyo3(signature = (other, /))]
    fn relate_to(&self, other: &PyAny) -> PyResult<&PyAny> {
        if other.is_instance_of::<PyExactContour>() {
            try_relation_to_py_relation(
                self.0
                    .relate_to(&other.extract::<PyRef<PyExactContour>>()?.0),
            )
        } else if other.is_instance_of::<PyExactEmpty>() {
            try_relation_to_py_relation(
                self.0.relate_to(&other.extract::<PyRef<PyExactEmpty>>()?.0),
            )
        } else if other.is_instance_of::<PyExactMultipolygon>() {
            try_relation_to_py_relation(
                self.0.relate_to(
                    &other.extract::<PyRef<PyExactMultipolygon>>()?.0,
                ),
            )
        } else if other.is_instance_of::<PyExactMultisegment>() {
            try_relation_to_py_relation(
                self.0.relate_to(
                    &other.extract::<PyRef<PyExactMultisegment>>()?.0,
                ),
            )
        } else if other.is_instance_of::<PyExactPolygon>() {
            try_relation_to_py_relation(
                self.0
                    .relate_to(&other.extract::<PyRef<PyExactPolygon>>()?.0),
            )
        } else if other.is_instance_of::<PyExactSegment>() {
            try_relation_to_py_relation(
                self.0
                    .relate_to(&other.extract::<PyRef<PyExactSegment>>()?.0),
            )
        } else {
            Err(PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, _point: &PyExactPoint) -> bool {
        false
    }

    fn __hash__(&self) -> ffi::Py_hash_t {
        0
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            Ok(PyExactContour((&self.0).union(&other.0)).into_py(py))
        } else if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).union(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            Ok((&self.0).union(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            Ok(PyExactMultisegment((&self.0).union(&other.0)).into_py(py))
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            Ok((&self.0).union(&other.0).into_py(py))
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            Ok(PyExactSegment((&self.0).union(&other.0)).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self) -> String {
        format!("{}()", Self::NAME)
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __sub__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            Ok(PyExactEmpty((&self.0).difference(&other.0)).into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyRef<PyExactContour>>()?;
            Ok(PyExactContour((&self.0).symmetric_difference(&other.0))
                .into_py(py))
        } else if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).symmetric_difference(&other.0))
                .into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            Ok((&self.0).symmetric_difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            Ok(
                PyExactMultisegment((&self.0).symmetric_difference(&other.0))
                    .into_py(py),
            )
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            Ok((&self.0).symmetric_difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            Ok(PyExactSegment((&self.0).symmetric_difference(&other.0))
                .into_py(py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pymethods]
impl PyExactMultipolygon {
    #[new]
    #[pyo3(signature = (polygons, /))]
    fn new(polygons: &PySequence) -> PyResult<Self> {
        try_polygons_to_py_exact_multipolygon(extract_from_py_sequence::<
            PyExactPolygon,
            ExactPolygon,
        >(polygons)?)
    }

    #[getter]
    fn bounding_box(&self) -> ExactBox {
        (&self.0).to_bounding_box().cloned()
    }

    #[getter]
    fn polygons(&self) -> Vec<ExactPolygon> {
        (&self.0).polygons().into_iter().cloned().collect()
    }

    #[getter]
    fn polygons_count(&self) -> usize {
        (&self.0).polygons().len()
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).intersection(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).intersection(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyFrozenSet::new(py, &self.polygons())?.hash()
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).union(other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).union(&other.0);
            debug_assert!(!polygons.is_empty());
            match polygons.len() {
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).union(&other.0);
            debug_assert!(!polygons.is_empty());
            match polygons.len() {
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            Self::NAME,
            self.polygons()
                .into_py(py)
                .as_ref(py)
                .repr()?
                .extract::<String>()?
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}([{}])",
            Self::NAME,
            self.polygons()
                .into_iter()
                .map(|polygon| PyExactPolygon(polygon).__str__())
                .collect::<PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).symmetric_difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).symmetric_difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).symmetric_difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pymethods]
impl PyExactMultisegment {
    #[new]
    #[pyo3(signature = (segments, /))]
    fn new(segments: &PySequence) -> PyResult<Self> {
        try_segments_to_py_exact_multisegment(extract_from_py_sequence::<
            PyExactSegment,
            ExactSegment,
        >(segments)?)
    }

    #[getter]
    fn bounding_box(&self) -> ExactBox {
        (&self.0).to_bounding_box().cloned()
    }

    #[getter]
    fn segments(slf: PyRef<Self>) -> PyExactMultisegmentSegments {
        let segments_count = (&slf.0).segments().len();
        PyExactMultisegmentSegments {
            multisegment: PyExactMultisegmentReference::from_py_ref(slf),
            start: 0isize,
            stop: segments_count as isize,
            step: 1isize,
        }
    }

    #[getter]
    fn segments_count(&self) -> usize {
        (&self.0).segments().len()
    }

    fn is_valid(&self) -> bool {
        is_multisegment_valid(&self.0)
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __and__(&self, other: &PyAny, py: Python) -> PyResult<PyObject> {
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            let segments = (&self.0).intersection(&other.0);
            match segments.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    segments.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultisegment(ExactMultisegment::new(segments))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            let segments = (&self.0).intersection(&other.0);
            match segments.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    segments.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultisegment(ExactMultisegment::new(segments))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyFrozenSet::new(py, (&self.0).segments())?.hash()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "{}({})",
            Self::NAME,
            (&self.0)
                .segments()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .into_py(py)
                .as_ref(py)
                .repr()?
                .extract::<String>()?
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}([{}])",
            Self::NAME,
            (&self.0)
                .segments()
                .iter()
                .cloned()
                .map(|segment| PyExactSegment(segment).__str__())
                .collect::<PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }
}

#[pymethods]
impl PyExactPoint {
    #[new]
    #[pyo3(signature = (x, y, /))]
    fn new(x: &PyAny, y: &PyAny) -> PyResult<Self> {
        Ok(PyExactPoint(ExactPoint::new(
            try_scalar_to_fraction(x)?,
            try_scalar_to_fraction(y)?,
        )))
    }

    #[getter]
    fn x(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction((&self.0).x())
    }

    #[getter]
    fn y(&self) -> PyResult<&PyAny> {
        try_fraction_to_py_fraction((&self.0).y())
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyTuple::new(py, [self.x()?, self.y()?]).hash()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.x()?.repr()?.extract::<String>()?,
            self.y()?.repr()?.extract::<String>()?,
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactPoint::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPoint>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ge => Ok((self.0 >= other.0).into_py(py)),
                CompareOp::Gt => Ok((self.0 > other.0).into_py(py)),
                CompareOp::Le => Ok((self.0 <= other.0).into_py(py)),
                CompareOp::Lt => Ok((self.0 < other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.x()?.str()?.extract::<String>()?,
            self.y()?.str()?.extract::<String>()?,
        ))
    }
}

#[pymethods]
impl PyExactPolygon {
    #[new]
    #[pyo3(signature = (border, holes, /))]
    fn new(border: &PyExactContour, holes: &PySequence) -> PyResult<Self> {
        Ok(PyExactPolygon(ExactPolygon::new(
            border.0.clone(),
            extract_from_py_sequence::<PyExactContour, ExactContour>(holes)?,
        )))
    }

    #[getter]
    fn border(&self) -> ExactContour {
        (&self.0).border().clone()
    }

    #[getter]
    fn bounding_box(&self) -> ExactBox {
        (&self.0).to_bounding_box().cloned()
    }

    #[getter]
    fn holes(slf: PyRef<Self>) -> PyExactPolygonHoles {
        PyExactPolygonHoles(PyExactPolygonReference::from_py_ref(slf))
    }

    #[getter]
    fn holes_count(&self) -> usize {
        (&self.0).holes().len()
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __and__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).intersection(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).intersection(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyTuple::new(
            py,
            &[
                self.border().into_py(py),
                PyFrozenSet::new(py, (&self.0).holes())?.into_py(py),
            ],
        )
        .hash()
    }

    fn __or__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).union(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).union(&other.0);
            debug_assert!(!polygons.is_empty());
            match polygons.len() {
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).union(&other.0);
            debug_assert!(!polygons.is_empty());
            match polygons.len() {
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            PyExactContour(self.border()).__repr__()?,
            (&self.0)
                .holes()
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .into_py(py)
                .as_ref(py)
                .repr()?
                .extract::<String>()?
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, [{}])",
            Self::NAME,
            PyExactContour(self.border()).__str__()?,
            (&self.0)
                .holes()
                .iter()
                .cloned()
                .map(|hole| PyExactContour(hole).__str__())
                .collect::<PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &PyAny) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok((&self.0).symmetric_difference(&other.0).into_py(py))
        } else if other.is_instance(PyExactMultipolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultipolygon>>()?;
            let polygons = (&self.0).symmetric_difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyRef<PyExactPolygon>>()?;
            let polygons = (&self.0).symmetric_difference(&other.0);
            match polygons.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    polygons.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons))
                    .into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pymethods]
impl PyExactSegment {
    #[new]
    #[pyo3(signature = (start, end, /))]
    fn new(start: &PyExactPoint, end: &PyExactPoint) -> Self {
        PyExactSegment(ExactSegment::new(start.0.clone(), end.0.clone()))
    }

    #[getter]
    fn bounding_box(&self) -> ExactBox {
        (&self.0).to_bounding_box().cloned()
    }

    #[getter]
    fn end(&self) -> PyExactPoint {
        PyExactPoint((&self.0).end().clone())
    }

    #[getter]
    fn start(&self) -> PyExactPoint {
        PyExactPoint((&self.0).start().clone())
    }

    #[pyo3(signature = (other, /))]
    fn relate_to(&self, other: &PyAny) -> PyResult<&PyAny> {
        if other.is_instance_of::<PyExactContour>() {
            try_relation_to_py_relation(
                self.0
                    .relate_to(&other.extract::<PyRef<PyExactContour>>()?.0),
            )
        } else if other.is_instance_of::<PyExactEmpty>() {
            try_relation_to_py_relation(
                self.0.relate_to(&other.extract::<PyRef<PyExactEmpty>>()?.0),
            )
        } else if other.is_instance_of::<PyExactMultisegment>() {
            try_relation_to_py_relation(
                self.0.relate_to(
                    &other.extract::<PyRef<PyExactMultisegment>>()?.0,
                ),
            )
        } else if other.is_instance_of::<PyExactSegment>() {
            try_relation_to_py_relation(
                self.0
                    .relate_to(&other.extract::<PyRef<PyExactSegment>>()?.0),
            )
        } else {
            Err(PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __and__(&self, other: &PyAny, py: Python) -> PyResult<PyObject> {
        if other.is_instance(PyExactEmpty::type_object(py))? {
            let other = other.extract::<PyRef<PyExactEmpty>>()?;
            Ok(PyExactEmpty((&self.0).intersection(&other.0)).into_py(py))
        } else if other.is_instance(PyExactMultisegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactMultisegment>>()?;
            let segments = (&self.0).intersection(&other.0);
            match segments.len() {
                0 => Ok(PyExactEmpty::new().into_py(py)),
                1 => Ok(unsafe {
                    segments.into_iter().next().unwrap_unchecked()
                }
                .into_py(py)),
                _ => Ok(PyExactMultisegment(ExactMultisegment::new(segments))
                    .into_py(py)),
            }
        } else if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            match (&self.0).intersection(&other.0) {
                None => Ok(PyExactEmpty::new().into_py(py)),
                Some(segment) => Ok(segment.into_py(py)),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyFrozenSet::new(
            py,
            &[self.start().into_py(py), self.end().into_py(py)],
        )?
        .hash()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.start().__repr__()?,
            self.end().__repr__()?,
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyRef<PyExactSegment>>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            Self::NAME,
            self.start().__str__()?,
            self.end().__str__()?,
        ))
    }
}

#[pymethods]
impl PyExactTrapezoidation {
    #[classmethod]
    #[pyo3(signature = (multisegment, seed, /))]
    fn from_multisegment(
        _: &PyType,
        multisegment: &PyExactMultisegment,
        seed: usize,
    ) -> Self {
        PyExactTrapezoidation(Trapezoidation::from_multisegment(
            &multisegment.0,
            |values| permute(values, seed),
        ))
    }

    #[classmethod]
    #[pyo3(signature = (polygon, seed, /))]
    fn from_polygon(
        _: &PyType,
        polygon: &PyExactPolygon,
        seed: usize,
    ) -> Self {
        PyExactTrapezoidation(Trapezoidation::from_polygon(
            &polygon.0,
            |values| {
                permute(values, seed);
            },
        ))
    }

    #[getter]
    fn height(&self) -> usize {
        self.0.height()
    }

    #[pyo3(signature = (point, /))]
    fn locate(&self, point: &PyExactPoint) -> PyResult<&PyAny> {
        try_location_to_py_location(self.0.locate(&point.0))
    }

    fn __contains__(&self, point: &PyExactPoint) -> bool {
        self.0.locate(&point.0) != Location::Exterior
    }
}

fn try_fraction_to_repr(value: &Fraction) -> PyResult<&PyString> {
    try_fraction_to_py_fraction(value)?.repr()
}

fn try_fraction_to_py_fraction<'a>(value: &Fraction) -> PyResult<&'a PyAny> {
    let fraction_cls = unsafe { MAYBE_FRACTION_CLS.unwrap_unchecked() };
    fraction_cls.call(
        (
            big_int_to_py_long(value.numerator()),
            big_int_to_py_long(value.denominator()),
        ),
        None,
    )
}

fn try_location_to_py_location<'a>(value: Location) -> PyResult<&'a PyAny> {
    let location_cls = unsafe { MAYBE_LOCATION_CLS.unwrap_unchecked() };
    let py = location_cls.py();
    location_cls.getattr(match value {
        Location::Boundary => intern!(py, "BOUNDARY"),
        Location::Exterior => intern!(py, "EXTERIOR"),
        Location::Interior => intern!(py, "INTERIOR"),
    })
}

fn try_relation_to_py_relation<'a>(value: Relation) -> PyResult<&'a PyAny> {
    let relation_cls = unsafe { MAYBE_RELATION_CLS.unwrap_unchecked() };
    let py = relation_cls.py();
    relation_cls.getattr(match value {
        Relation::Component => intern!(py, "COMPONENT"),
        Relation::Composite => intern!(py, "COMPOSITE"),
        Relation::Cover => intern!(py, "COVER"),
        Relation::Cross => intern!(py, "CROSS"),
        Relation::Disjoint => intern!(py, "DISJOINT"),
        Relation::Enclosed => intern!(py, "ENCLOSED"),
        Relation::Encloses => intern!(py, "ENCLOSES"),
        Relation::Equal => intern!(py, "EQUAL"),
        Relation::Overlap => intern!(py, "OVERLAP"),
        Relation::Touch => intern!(py, "TOUCH"),
        Relation::Within => intern!(py, "WITHIN"),
    })
}

#[inline]
fn big_int_to_py_long(value: &BigInt) -> PyObject {
    let buffer = value.to_bytes(Endianness::Little);
    Python::with_gil(|py| unsafe {
        PyObject::from_owned_ptr(
            py,
            ffi::_PyLong_FromByteArray(buffer.as_ptr(), buffer.len(), 1, 1),
        )
    })
}

#[inline]
fn try_py_integral_to_big_int(value: &PyAny) -> PyResult<BigInt> {
    let ptr = value.as_ptr();
    let py = value.py();
    unsafe {
        let ptr = ffi::PyNumber_Long(ptr);
        if ptr.is_null() {
            return Err(PyErr::fetch(py));
        }
        let bits_count = ffi::_PyLong_NumBits(ptr);
        match bits_count.cmp(&0) {
            Ordering::Less => Err(PyErr::fetch(py)),
            Ordering::Equal => Ok(BigInt::zero()),
            Ordering::Greater => {
                let bytes_count = bits_count / (u8::BITS as usize) + 1;
                let mut buffer = vec![0u8; bytes_count];
                if ffi::_PyLong_AsByteArray(
                    Py::<PyLong>::from_owned_ptr(py, ptr)
                        .as_ptr()
                        .cast::<ffi::PyLongObject>(),
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    1,
                    1,
                ) < 0
                {
                    Err(PyErr::fetch(py))
                } else {
                    Ok(BigInt::from_bytes(
                        buffer.as_mut_slice(),
                        Endianness::Little,
                    ))
                }
            }
        }
    }
}

const INVALID_SCALAR_TYPE_ERROR_MESSAGE: &str =
    "Scalar should be a rational number.";
const UNDEFINED_DIVISION_ERROR_MESSAGE: &str =
    "Division by zero is undefined.";

fn try_scalar_to_fraction(value: &PyAny) -> PyResult<Fraction> {
    let py = value.py();
    if value.is_instance(PyFloat::type_object(py))? {
        Fraction::try_from(value.extract::<f64>()?).map_err(|reason| {
            match reason {
                fraction::FromFloatConstructionError::Infinity => {
                    PyOverflowError::new_err(reason.to_string())
                }
                _ => PyValueError::new_err(reason.to_string()),
            }
        })
    } else {
        let numerator = try_py_integral_to_big_int(
            value.getattr(intern!(py, "numerator")).map_err(|_| {
                PyTypeError::new_err(INVALID_SCALAR_TYPE_ERROR_MESSAGE)
            })?,
        )?;
        let denominator = try_py_integral_to_big_int(
            value.getattr(intern!(py, "denominator")).map_err(|_| {
                PyTypeError::new_err(INVALID_SCALAR_TYPE_ERROR_MESSAGE)
            })?,
        )?;
        match Fraction::new(numerator, denominator) {
            Some(value) => Ok(value),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }
}

fn try_vertices_to_py_exact_contour(
    vertices: Vec<ExactPoint>,
) -> PyResult<PyExactContour> {
    if vertices.len() < MIN_CONTOUR_VERTICES_COUNT {
        Err(PyValueError::new_err(format!(
            "Contour should have at least {} vertices, but found {}.",
            MIN_CONTOUR_VERTICES_COUNT,
            vertices.len()
        )))
    } else {
        Ok(PyExactContour(ExactContour::new(vertices)))
    }
}

fn try_polygons_to_py_exact_multipolygon(
    polygons: Vec<ExactPolygon>,
) -> PyResult<PyExactMultipolygon> {
    if polygons.len() < MIN_MULTIPOLYGON_POLYGONS_COUNT {
        Err(PyValueError::new_err(format!(
            "Multipolygon should have at least {} polygons, but found {}.",
            MIN_MULTIPOLYGON_POLYGONS_COUNT,
            polygons.len()
        )))
    } else {
        Ok(PyExactMultipolygon(ExactMultipolygon::new(polygons)))
    }
}

fn try_segments_to_py_exact_multisegment(
    segments: Vec<ExactSegment>,
) -> PyResult<PyExactMultisegment> {
    if segments.len() < MIN_MULTISEGMENT_SEGMENTS_COUNT {
        Err(PyValueError::new_err(format!(
            "Multisegment should have at least {} segments, but found {}.",
            MIN_MULTISEGMENT_SEGMENTS_COUNT,
            segments.len()
        )))
    } else {
        Ok(PyExactMultisegment(ExactMultisegment::new(segments)))
    }
}

static mut MAYBE_FRACTION_CLS: Option<&PyAny> = None;
static mut MAYBE_ORIENTATION_CLS: Option<&PyAny> = None;
static mut MAYBE_LOCATION_CLS: Option<&PyAny> = None;
static mut MAYBE_RELATION_CLS: Option<&PyAny> = None;

fn extract_from_py_sequence<
    'a,
    Wrapper: FromPyObject<'a>,
    Wrapped: From<Wrapper>,
>(
    sequence: &'a PySequence,
) -> PyResult<Vec<Wrapped>> {
    let mut result = Vec::<Wrapped>::with_capacity(sequence.len()?);
    for element in sequence.iter()? {
        result.push(element?.extract::<Wrapper>()?.into());
    }
    Ok(result)
}

fn sequence_get_item<T: Clone, SequenceT: Sequence<IndexItem = T>>(
    sequence: &SequenceT,
    item: &PyAny,
) -> PyResult<T> {
    let maybe_index = unsafe { ffi::PyNumber_Index(item.as_ptr()) };
    if maybe_index.is_null() {
        Err(PyErr::fetch(item.py()))
    } else {
        Ok(sequence[py_long_to_valid_index(
            unsafe { PyLong::from_owned_ptr(item.py(), maybe_index) },
            sequence.len(),
        )?]
        .clone())
    }
}

fn py_long_to_valid_index(
    value: &PyLong,
    elements_count: usize,
) -> PyResult<usize> {
    if let Ok(index) = value.extract::<isize>() {
        let elements_count = elements_count as isize;
        if !(-elements_count <= index && index < elements_count) {
            Err(PyIndexError::new_err(format!(
                "Index {} is out of range({}, {}).",
                index, -elements_count, elements_count
            )))
        } else {
            Ok((if index < 0 {
                index + elements_count
            } else {
                index
            }) as usize)
        }
    } else {
        Err(PyIndexError::new_err(format!(
            "Index {} is out of index integer range({}, {}).",
            value.repr()?,
            isize::MIN,
            (isize::MAX as usize) + 1
        )))
    }
}

#[pymodule]
fn _cexact(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyExactBox>()?;
    module.add_class::<PyExactConstrainedDelaunayTriangulation>()?;
    module.add_class::<PyExactContour>()?;
    module.add_class::<PyExactDelaunayTriangulation>()?;
    module.add_class::<PyExactEmpty>()?;
    module.add_class::<PyExactTrapezoidation>()?;
    module.add_class::<PyExactMultipolygon>()?;
    module.add_class::<PyExactMultisegment>()?;
    module.add_class::<PyExactPoint>()?;
    module.add_class::<PyExactPolygon>()?;
    module.add_class::<PyExactSegment>()?;
    PySequence::register::<PyExactContourSegments>(py)?;
    let collections_abc_module = py.import("collections.abc")?;
    let sequence_cls =
        collections_abc_module.getattr(intern!(py, "Sequence"))?;
    sequence_cls.call_method1(
        "register",
        (PyExactContourVertices::type_object(py),),
    )?;
    sequence_cls.call_method1(
        "register",
        (PyExactMultisegmentSegments::type_object(py),),
    )?;
    sequence_cls
        .call_method1("register", (PyExactPolygonHoles::type_object(py),))?;
    unsafe {
        let py = Python::assume_gil_acquired();
        MAYBE_FRACTION_CLS = Some(
            py.import("rithm.fraction")?
                .getattr(intern!(py, "Fraction"))?,
        );
        MAYBE_LOCATION_CLS =
            Some(py.import("rene")?.getattr(intern!(py, "Location"))?);
        MAYBE_ORIENTATION_CLS =
            Some(py.import("rene")?.getattr(intern!(py, "Orientation"))?);
        MAYBE_RELATION_CLS =
            Some(py.import("rene")?.getattr(intern!(py, "Relation"))?);
    }
    Ok(())
}

#[pymodule]
fn _crene(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyLocation>()?;
    module.add_class::<PyOrientation>()?;
    module.add_class::<PyRelation>()?;
    module.add("MIN_CONTOUR_VERTICES_COUNT", MIN_CONTOUR_VERTICES_COUNT)?;
    module.add(
        "MIN_MULTIPOLYGON_POLYGONS_COUNT",
        MIN_MULTIPOLYGON_POLYGONS_COUNT,
    )?;
    module.add(
        "MIN_MULTISEGMENT_SEGMENTS_COUNT",
        MIN_MULTISEGMENT_SEGMENTS_COUNT,
    )?;
    Ok(())
}
