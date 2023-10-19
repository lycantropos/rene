use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::{Add, Mul, Sub};

use rithm::{big_int, fraction};
use traiter::numbers::{Endianness, FromBytes, Sign, Signed, ToBytes, Zero};

use crate::locatable::Location;
use crate::operations::{
    CrossMultiply, DotMultiply, LocatePointInPointPointPointCircle, Square,
    SquaredMetric,
};
use crate::traits::{
    Elemental, Multipolygonal, Multisegmental, Multivertexal, Polygonal,
};

use super::impl_box_wrapper::impl_box_wrapper;
use super::impl_constrained_delaunay_triangulation_wrapper::impl_constrained_delaunay_triangulation_wrapper;
use super::impl_contour_wrapper::impl_contour_wrapper;
use super::impl_delaunay_triangulation_wrapper::impl_delaunay_triangulation_wrapper;
use super::impl_empty_wrapper::impl_empty_wrapper;
use super::impl_multipolygon_wrapper::impl_multipolygon_wrapper;
use super::impl_multisegment_wrapper::impl_multisegment_wrapper;
use super::impl_point_wrapper::impl_point_wrapper;
use super::impl_polygon_wrapper::impl_polygon_wrapper;
use super::impl_py_sequence::impl_py_sequence;
use super::impl_segment_wrapper::impl_segment_wrapper;
use super::impl_trapezoidation_wrapper::impl_trapezoidation_wrapper;
use super::reference;
use super::traits::{TryFromPyAny, TryToPyAny};

#[pyo3::prelude::pymodule]
fn _cexact(
    py: pyo3::Python,
    module: &pyo3::types::PyModule,
) -> pyo3::PyResult<()> {
    module.add_class::<PyBox>()?;
    module.add_class::<PyConstrainedDelaunayTriangulation>()?;
    module.add_class::<PyContour>()?;
    module.add_class::<PyDelaunayTriangulation>()?;
    module.add_class::<PyEmpty>()?;
    module.add_class::<PyTrapezoidation>()?;
    module.add_class::<PyMultipolygon>()?;
    module.add_class::<PyMultisegment>()?;
    module.add_class::<PyPoint>()?;
    module.add_class::<PyPolygon>()?;
    module.add_class::<PySegment>()?;
    pyo3::types::PySequence::register::<PyContourSegments>(py)?;
    pyo3::types::PySequence::register::<PyContourVertices>(py)?;
    pyo3::types::PySequence::register::<PyMultipolygonPolygons>(py)?;
    pyo3::types::PySequence::register::<PyMultisegmentSegments>(py)?;
    pyo3::types::PySequence::register::<PyPolygonHoles>(py)?;
    Ok(())
}

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const DIGIT_BITNESS: usize = (Digit::BITS - 1) as usize;
const _: () =
    assert!(big_int::is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());

type BigInt = big_int::BigInt<Digit, DIGIT_BITNESS>;
type Fraction = fraction::Fraction<BigInt>;

impl Default for PyEmpty {
    fn default() -> Self {
        PyEmpty(Empty::new())
    }
}

impl From<Vec<Polygon>> for PyMultipolygon {
    fn from(value: Vec<Polygon>) -> Self {
        Self(Multipolygon::new(value))
    }
}

impl From<Vec<Point>> for PyContour {
    fn from(value: Vec<Point>) -> Self {
        Self(Contour::new(value))
    }
}

impl From<Vec<Segment>> for PyMultisegment {
    fn from(value: Vec<Segment>) -> Self {
        Self(Multisegment::new(value))
    }
}

impl<Point> CrossMultiply for &Point
where
    Fraction: Mul<Output = Fraction> + Sub<Output = Fraction>,
    for<'a> &'a Fraction: Sub<Output = Fraction>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Fraction>,
{
    type Output = Fraction;

    fn cross_multiply(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output {
        let (first_start_x, first_start_y) = first_start.coordinates();
        let (first_end_x, first_end_y) = first_end.coordinates();
        let (second_start_x, second_start_y) = second_start.coordinates();
        let (second_end_x, second_end_y) = second_end.coordinates();
        (first_end_x - first_start_x) * (second_end_y - second_start_y)
            - (first_end_y - first_start_y) * (second_end_x - second_start_x)
    }
}

impl<Point> DotMultiply for &Point
where
    Fraction: Add<Output = Fraction> + Mul<Output = Fraction>,
    for<'a> &'a Fraction: Sub<Output = Fraction>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Fraction>,
{
    type Output = Fraction;

    fn dot_multiply(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output {
        let (first_start_x, first_start_y) = first_start.coordinates();
        let (first_end_x, first_end_y) = first_end.coordinates();
        let (second_start_x, second_start_y) = second_start.coordinates();
        let (second_end_x, second_end_y) = second_end.coordinates();
        (first_end_x - first_start_x) * (second_end_x - second_start_x)
            + (first_end_y - first_start_y) * (second_end_y - second_start_y)
    }
}

impl<'a, Point> LocatePointInPointPointPointCircle for &'a Point
where
    &'a Point: Elemental<Coordinate = &'a Fraction>,
    Fraction: Add<Output = Fraction>
        + Mul<Output = Fraction>
        + Sub<Output = Fraction>,
    for<'b> &'b Fraction:
        Mul<Output = Fraction> + Signed + Sub<Output = Fraction>,
{
    fn locate_point_in_point_point_point_circle(
        self,
        first: Self,
        second: Self,
        third: Self,
    ) -> Location {
        let (first_dx, first_dy) =
            (first.x() - self.x(), first.y() - self.y());
        let (second_dx, second_dy) =
            (second.x() - self.x(), second.y() - self.y());
        let (third_dx, third_dy) =
            (third.x() - self.x(), third.y() - self.y());
        match ((&first_dx * &first_dx + &first_dy * &first_dy)
            * (&second_dx * &third_dy - &second_dy * &third_dx)
            - (&second_dx * &second_dx + &second_dy * &second_dy)
                * (&first_dx * &third_dy - &first_dy * &third_dx)
            + (&third_dx * &third_dx + &third_dy * &third_dy)
                * (first_dx * second_dy - first_dy * second_dx))
            .sign()
        {
            Sign::Negative => Location::Exterior,
            Sign::Positive => Location::Interior,
            Sign::Zero => Location::Boundary,
        }
    }
}

impl Square for Fraction
where
    Fraction: Clone + Mul<Output = Fraction>,
{
    type Output = Self;

    fn square(self) -> Self::Output {
        self.clone() * self
    }
}

impl<Point> SquaredMetric for &Point
where
    Fraction: Add<Output = Fraction> + Square<Output = Fraction>,
    for<'a> &'a Fraction: Sub<Output = Fraction>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Fraction>,
{
    type Output = Fraction;

    fn squared_distance_to(self, other: Self) -> Self::Output {
        let (start_x, start_y) = self.coordinates();
        let (other_start_x, other_start_y) = other.coordinates();
        (start_x - other_start_x).square() + (start_y - other_start_y).square()
    }
}

const INVALID_SCALAR_TYPE_ERROR_MESSAGE: &str =
    "Scalar should be a rational number.";
const UNDEFINED_DIVISION_ERROR_MESSAGE: &str =
    "Division by zero is undefined.";

impl TryFromPyAny for Fraction {
    fn try_from_py_any(
        value: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<Self> {
        if value.is_instance(<pyo3::types::PyFloat as pyo3::type_object::PyTypeInfo>::type_object(py))? {
                Fraction::try_from(value.extract::<f64>()?).map_err(|reason| {
                    match reason {
                        fraction::FromFloatConstructionError::Infinity => {
                            pyo3::exceptions::PyOverflowError::new_err(reason.to_string())
                        }
                        _ => pyo3::exceptions::PyValueError::new_err(reason.to_string()),
                    }
                })
            } else {
                let numerator = try_py_integral_to_big_int(
                    value.getattr(pyo3::intern!(py, "numerator")).map_err(
                        |_| {
                            pyo3::exceptions::PyTypeError::new_err(
                                INVALID_SCALAR_TYPE_ERROR_MESSAGE,
                            )
                        },
                    )?,
                )?;
                let denominator = try_py_integral_to_big_int(
                    value.getattr(pyo3::intern!(py, "denominator")).map_err(
                        |_| {
                            pyo3::exceptions::PyTypeError::new_err(
                                INVALID_SCALAR_TYPE_ERROR_MESSAGE,
                            )
                        },
                    )?,
                )?;
                match Fraction::new(numerator, denominator) {
                    Some(value) => Ok(value),
                    None => Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                        UNDEFINED_DIVISION_ERROR_MESSAGE,
                    )),
                }
            }
    }
}

impl TryToPyAny for &Fraction {
    fn try_to_py_any(
        self,
        py: pyo3::Python<'_>,
    ) -> pyo3::PyResult<&'_ pyo3::PyAny> {
        static FRACTION_CLS: pyo3::sync::GILOnceCell<pyo3::PyObject> =
            pyo3::sync::GILOnceCell::new();
        FRACTION_CLS
            .get_or_try_init(py, || {
                py.import("rithm.fraction")?
                    .getattr(pyo3::intern!(py, "Fraction"))
                    .map(|value| pyo3::IntoPy::into_py(value, py))
            })?
            .call(
                py,
                (
                    big_int_to_py_long(self.numerator()),
                    big_int_to_py_long(self.denominator()),
                ),
                None,
            )
            .map(|value| value.into_ref(py))
    }
}

impl pyo3::ToPyObject for Contour {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(self.clone(), py)
    }
}

impl pyo3::ToPyObject for Point {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(self.clone(), py)
    }
}

impl pyo3::ToPyObject for Polygon {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(self.clone(), py)
    }
}

impl pyo3::ToPyObject for Segment {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(self.clone(), py)
    }
}

impl From<PyContour> for Contour {
    fn from(value: PyContour) -> Self {
        value.0
    }
}

impl From<PyPoint> for Point {
    fn from(value: PyPoint) -> Self {
        value.0
    }
}

impl From<PyPolygon> for Polygon {
    fn from(value: PyPolygon) -> Self {
        value.0
    }
}

impl From<PySegment> for Segment {
    fn from(value: PySegment) -> Self {
        value.0
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Box {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PyBox(self), py)
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Contour {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PyContour(self), py)
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Multipolygon {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PyMultipolygon(self), py)
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Point {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PyPoint(self), py)
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Polygon {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PyPolygon(self), py)
    }
}

impl pyo3::IntoPy<pyo3::PyObject> for Segment {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(PySegment(self), py)
    }
}

type Box = crate::bounded::Box<Fraction>;
type ConstrainedDelaunayTriangulation =
    crate::triangulation::ConstrainedDelaunayTriangulation<Point>;
type Contour = crate::geometries::Contour<Fraction>;
type DelaunayTriangulation =
    crate::triangulation::DelaunayTriangulation<Point>;
type Empty = crate::geometries::Empty;
type Multipolygon = crate::geometries::Multipolygon<Fraction>;
type Multisegment = crate::geometries::Multisegment<Fraction>;
type Point = crate::geometries::Point<Fraction>;
type Polygon = crate::geometries::Polygon<Fraction>;
type Segment = crate::geometries::Segment<Fraction>;
type Trapezoidation = crate::seidel::Trapezoidation<Point>;

#[pyo3::prelude::pyclass(name = "BaseBox", module = "rene.exact")]
#[derive(Clone)]
struct PyBox(Box);

#[pyo3::prelude::pyclass(
    name = "ConstrainedDelaunayTriangulation",
    module = "rene.exact"
)]
#[derive(Clone)]
struct PyConstrainedDelaunayTriangulation(ConstrainedDelaunayTriangulation);

#[pyo3::prelude::pyclass(name = "Contour", module = "rene.exact")]
#[derive(Clone)]
struct PyContour(Contour);

#[pyo3::prelude::pyclass(
    name = "DelaunayTriangulation",
    module = "rene.exact"
)]
#[derive(Clone)]
struct PyDelaunayTriangulation(DelaunayTriangulation);

#[pyo3::prelude::pyclass(name = "Empty", module = "rene.exact")]
#[derive(Clone)]
struct PyEmpty(Empty);

#[pyo3::prelude::pyclass(name = "Multipolygon", module = "rene.exact")]
#[derive(Clone)]
struct PyMultipolygon(Multipolygon);

#[pyo3::prelude::pyclass(name = "Multisegment", module = "rene.exact")]
#[derive(Clone)]
struct PyMultisegment(Multisegment);

#[pyo3::prelude::pyclass(name = "Polygon", module = "rene.exact")]
#[derive(Clone)]
struct PyPolygon(Polygon);

#[pyo3::prelude::pyclass(name = "Point", module = "rene.exact")]
#[derive(Clone)]
struct PyPoint(Point);

#[pyo3::prelude::pyclass(name = "Segment", module = "rene.exact")]
#[derive(Clone)]
struct PySegment(Segment);

#[pyo3::prelude::pyclass(name = "Trapezoidation", module = "rene.exact")]
#[derive(Clone)]
struct PyTrapezoidation(Trapezoidation);

impl_box_wrapper!();
impl_constrained_delaunay_triangulation_wrapper!();
impl_contour_wrapper!();
impl_delaunay_triangulation_wrapper!();
impl_empty_wrapper!();
impl_multipolygon_wrapper!();
impl_multisegment_wrapper!();
impl_point_wrapper!();
impl_polygon_wrapper!();
impl_segment_wrapper!();
impl_trapezoidation_wrapper!();

type PyContourReference = reference::Reference<PyContour>;
type PyMultisegmentReference = reference::Reference<PyMultisegment>;
type PyMultipolygonReference = reference::Reference<PyMultipolygon>;
type PyPolygonReference = reference::Reference<PyPolygon>;

#[pyo3::prelude::pyclass(
    module = "rene.exact",
    name = "_ContourSegments",
    sequence
)]
struct PyContourSegments {
    contour: PyContourReference,
    start: isize,
    stop: isize,
    step: isize,
}

#[pyo3::prelude::pyclass(
    module = "rene.exact",
    name = "_ContourVertices",
    sequence
)]
struct PyContourVertices {
    contour: PyContourReference,
    start: isize,
    stop: isize,
    step: isize,
}

#[pyo3::prelude::pyclass(
    module = "rene.exact",
    name = "_MultisegmentSegments",
    sequence
)]
struct PyMultisegmentSegments {
    multisegment: PyMultisegmentReference,
    start: isize,
    stop: isize,
    step: isize,
}

#[pyo3::prelude::pyclass(
    module = "rene.exact",
    name = "_MultipolygonPolygons",
    sequence
)]
struct PyMultipolygonPolygons {
    multipolygon: PyMultipolygonReference,
    start: isize,
    stop: isize,
    step: isize,
}

#[pyo3::prelude::pyclass(
    module = "rene.exact",
    name = "_PolygonHoles",
    sequence
)]
struct PyPolygonHoles {
    polygon: PyPolygonReference,
    start: isize,
    stop: isize,
    step: isize,
}

impl_py_sequence!(
    PyContourSegments,
    contour,
    segment,
    segments,
    PySegment,
    Segment
);

impl_py_sequence!(PyContourVertices, contour, point, vertices, PyPoint, Point);

impl_py_sequence!(
    PyMultisegmentSegments,
    multisegment,
    segment,
    segments,
    PySegment,
    Segment
);

impl_py_sequence!(
    PyMultipolygonPolygons,
    multipolygon,
    polygon,
    polygons,
    PyPolygon,
    Polygon
);

impl_py_sequence!(PyPolygonHoles, polygon, contour, holes, PyContour, Contour);

fn big_int_to_py_long(value: &BigInt) -> pyo3::PyObject {
    let buffer = value.to_bytes(Endianness::Little);
    pyo3::Python::with_gil(|py| unsafe {
        pyo3::PyObject::from_owned_ptr(
            py,
            pyo3::ffi::_PyLong_FromByteArray(
                buffer.as_ptr(),
                buffer.len(),
                1,
                1,
            ),
        )
    })
}

fn try_py_integral_to_big_int(value: &pyo3::PyAny) -> pyo3::PyResult<BigInt> {
    let ptr = pyo3::AsPyPointer::as_ptr(value);
    let py = value.py();
    unsafe {
        let ptr = pyo3::ffi::PyNumber_Long(ptr);
        if ptr.is_null() {
            return Err(pyo3::PyErr::fetch(py));
        }
        let bits_count = pyo3::ffi::_PyLong_NumBits(ptr);
        match bits_count.cmp(&0) {
            Ordering::Less => Err(pyo3::PyErr::fetch(py)),
            Ordering::Equal => Ok(BigInt::zero()),
            Ordering::Greater => {
                let bytes_count = bits_count / (u8::BITS as usize) + 1;
                let mut buffer = vec![0u8; bytes_count];
                if pyo3::ffi::_PyLong_AsByteArray(
                    pyo3::AsPyPointer::as_ptr(
                        &pyo3::Py::<pyo3::types::PyLong>::from_owned_ptr(
                            py, ptr,
                        ),
                    )
                    .cast::<pyo3::ffi::PyLongObject>(),
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    1,
                    1,
                ) < 0
                {
                    Err(pyo3::PyErr::fetch(py))
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
