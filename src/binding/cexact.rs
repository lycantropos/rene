use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ops::{Add, Deref, Mul, Sub};

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

use super::impl_py_sequence::impl_py_sequence;
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
    fn try_from_py_any(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        let py = value.py();
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

#[pyo3::prelude::pyclass(name = "Box", module = "rene.exact")]
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

#[pyo3::prelude::pymethods]
impl PyBox {
    #[new]
    #[pyo3(signature = (min_x, max_x, min_y, max_y, /))]
    fn new(
        min_x: &pyo3::PyAny,
        max_x: &pyo3::PyAny,
        min_y: &pyo3::PyAny,
        max_y: &pyo3::PyAny,
    ) -> pyo3::prelude::PyResult<Self> {
        Ok(Self(Box::new(
            TryFromPyAny::try_from_py_any(min_x)?,
            TryFromPyAny::try_from_py_any(max_x)?,
            TryFromPyAny::try_from_py_any(min_y)?,
            TryFromPyAny::try_from_py_any(max_y)?,
        )))
    }

    #[getter]
    fn max_x<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(self.0.get_max_x(), py)
    }

    #[getter]
    fn max_y<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(self.0.get_max_y(), py)
    }

    #[getter]
    fn min_x<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(self.0.get_min_x(), py)
    }

    #[getter]
    fn min_y<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(self.0.get_min_y(), py)
    }

    #[pyo3(signature = (other, /))]
    fn covers(&self, other: &Self) -> bool {
        crate::relatable::Relatable::covers(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn disjoint_with(&self, other: &Self) -> bool {
        crate::relatable::Relatable::disjoint_with(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn enclosed_by(&self, other: &Self) -> bool {
        crate::relatable::Relatable::enclosed_by(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn encloses(&self, other: &Self) -> bool {
        crate::relatable::Relatable::encloses(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn equals_to(&self, other: &Self) -> bool {
        crate::relatable::Relatable::equals_to(&self.0, &other.0)
    }

    fn is_valid(&self) -> bool {
        self.0.get_min_x() <= self.0.get_max_x()
            && self.0.get_min_y() <= self.0.get_max_y()
    }

    #[pyo3(signature = (other, /))]
    fn overlaps(&self, other: &Self) -> bool {
        crate::relatable::Relatable::overlaps(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &Self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::relatable::Relatable::relate_to(&self.0, &other.0),
            py,
        )
    }

    #[pyo3(signature = (other, /))]
    fn touches(&self, other: &Self) -> bool {
        crate::relatable::Relatable::touches(&self.0, &other.0)
    }

    #[pyo3(signature = (other, /))]
    fn within(&self, other: &Self) -> bool {
        crate::relatable::Relatable::within(&self.0, &other.0)
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyTuple::new(
            py,
            [
                self.min_x(py)?,
                self.max_x(py)?,
                self.min_y(py)?,
                self.max_y(py)?,
            ],
        )
        .hash()
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.min_x(py)?.repr()?.extract::<String>()?,
            self.max_x(py)?.repr()?.extract::<String>()?,
            self.min_y(py)?.repr()?.extract::<String>()?,
            self.max_y(py)?.repr()?.extract::<String>()?,
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {}, {}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.min_x(py)?.str()?.extract::<String>()?,
            self.max_x(py)?.str()?.extract::<String>()?,
            self.min_y(py)?.str()?.extract::<String>()?,
            self.max_y(py)?.str()?.extract::<String>()?,
        ))
    }
}

#[pyo3::prelude::pymethods]
impl PyConstrainedDelaunayTriangulation {
    #[classmethod]
    #[pyo3(signature = (polygon, /))]
    fn from_polygon(_: &pyo3::types::PyType, polygon: &PyPolygon) -> Self {
        PyConstrainedDelaunayTriangulation(
            ConstrainedDelaunayTriangulation::from(&polygon.0),
        )
    }

    #[getter]
    fn border(&self) -> pyo3::PyResult<PyContour> {
        super::packing::try_pack_vertices(
            crate::triangulation::BoundaryEndpoints::get_boundary_endpoints(
                &self.0,
            )
            .into_iter()
            .cloned()
            .collect(),
        )
    }

    #[getter]
    fn triangles(&self) -> Vec<Contour> {
        self.0
            .to_triangles_vertices()
            .map(|(first, second, third)| {
                Contour::from([first.clone(), second.clone(), third.clone()])
            })
            .collect()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }
}

#[pyo3::prelude::pymethods]
impl PyContour {
    #[new]
    #[pyo3(signature = (vertices, /))]
    fn new(vertices: &pyo3::types::PySequence) -> pyo3::PyResult<Self> {
        super::packing::try_pack_vertices(
            super::conversion::extract_from_py_sequence::<Point, PyPoint>(
                vertices,
            )?,
        )
    }

    #[getter]
    fn bounding_box(&self) -> Box {
        crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
    }

    #[getter]
    fn segments(slf: pyo3::PyRef<Self>) -> PyContourSegments {
        let segments_count =
            crate::traits::Lengthsome::len(&(&slf.0).segments());
        PyContourSegments {
            contour: PyContourReference::from_py_ref(slf),
            start: 0isize,
            stop: segments_count as isize,
            step: 1isize,
        }
    }

    #[getter]
    fn vertices(slf: pyo3::PyRef<Self>) -> PyContourVertices {
        let vertices_count =
            crate::traits::Lengthsome::len(&(&slf.0).vertices());
        PyContourVertices {
            contour: PyContourReference::from_py_ref(slf),
            start: 0isize,
            stop: vertices_count as isize,
            step: 1isize,
        }
    }

    #[getter]
    fn orientation<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        crate::oriented::Oriented::to_orientation(&self.0).try_to_py_any(py)
    }

    fn is_valid(&self) -> bool {
        crate::bentley_ottmann::is_contour_valid(&self.0)
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultisegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<Self>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<Self>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PySegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PySegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyPolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultipolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultipolygon>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::ffi::Py_hash_t> {
        let mut vertices =
            (&self.0).vertices().into_iter().collect::<Vec<_>>();
        let min_vertex_index = unsafe {
            crate::operations::to_arg_min(&vertices).unwrap_unchecked()
        };
        vertices.rotate_left(min_vertex_index);
        if crate::oriented::Oriented::to_orientation(&self.0)
            == crate::oriented::Orientation::Clockwise
        {
            vertices[1..].reverse();
        }
        pyo3::types::PyTuple::new(py, &vertices).hash()
    }

    fn __or__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}([{}])",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            (&self.0)
                .vertices()
                .into_iter()
                .cloned()
                .map(|vertex| PyPoint(vertex).__repr__(py))
                .collect::<pyo3::PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "Contour([{}])",
            (&self.0)
                .vertices()
                .into_iter()
                .cloned()
                .map(|vertex| PyPoint(vertex).__str__(py))
                .collect::<pyo3::PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Difference::difference(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PyDelaunayTriangulation {
    #[classmethod]
    #[pyo3(signature = (points, /))]
    fn from_points(
        _: &pyo3::types::PyType,
        points: &pyo3::types::PySequence,
    ) -> pyo3::PyResult<Self> {
        Ok(PyDelaunayTriangulation(DelaunayTriangulation::from(
            super::conversion::extract_from_py_sequence::<Point, PyPoint>(
                points,
            )?,
        )))
    }

    #[getter]
    fn border(&self) -> pyo3::PyResult<PyContour> {
        super::packing::try_pack_vertices(
            crate::triangulation::BoundaryEndpoints::get_boundary_endpoints(
                &self.0,
            )
            .into_iter()
            .cloned()
            .collect(),
        )
    }

    #[getter]
    fn triangles(&self) -> Vec<Contour> {
        self.0
            .iter_triangles_vertices()
            .map(|(first, second, third)| {
                Contour::from([first.clone(), second.clone(), third.clone()])
            })
            .collect()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }
}

#[pyo3::prelude::pymethods]
impl PyEmpty {
    #[new]
    fn new() -> Self {
        PyEmpty(Empty::new())
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        crate::locatable::Location::Exterior.try_to_py_any(py)
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<PyContour>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultipolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultipolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultisegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyPolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PySegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PySegment>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, _point: &PyPoint) -> bool {
        false
    }

    fn __hash__(&self) -> pyo3::ffi::Py_hash_t {
        0
    }

    fn __or__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyContour(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Union::union(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyMultisegment(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Union::union(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PySegment(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self) -> String {
        format!("{}()", <Self as pyo3::type_object::PyTypeInfo>::NAME)
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __sub__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Difference::difference(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyContour(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                ),
                py,
            ))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyMultisegment(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                ),
                py,
            ))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            Ok(pyo3::IntoPy::into_py(
                PySegment(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PyMultipolygon {
    #[new]
    #[pyo3(signature = (polygons, /))]
    fn new(polygons: &pyo3::types::PySequence) -> pyo3::PyResult<Self> {
        super::packing::try_pack_polygons(
            super::conversion::extract_from_py_sequence::<Polygon, PyPolygon>(
                polygons,
            )?,
        )
    }

    #[getter]
    fn bounding_box(&self) -> Box {
        crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
    }

    #[getter]
    fn polygons(slf: pyo3::PyRef<Self>) -> PyMultipolygonPolygons {
        let polygons_count =
            crate::traits::Lengthsome::len(&(&slf.0).polygons());
        PyMultipolygonPolygons {
            multipolygon: PyMultipolygonReference::from_py_ref(slf),
            start: 0isize,
            stop: polygons_count as isize,
            step: 1isize,
        }
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<Self>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<Self>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyContour>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultisegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyPolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PySegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PySegment>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyFrozenSet::new(
            py,
            &(&self.0)
                .polygons()
                .into_iter()
                .cloned()
                .collect::<Vec<_>>(),
        )?
        .hash()
    }

    fn __or__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Union::union(&self.0, other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}({})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            pyo3::IntoPy::into_py(
                (&self.0)
                    .polygons()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<_>>(),
                py
            )
            .as_ref(py)
            .repr()?
            .extract::<String>()?
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}([{}])",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            (&self.0)
                .polygons()
                .into_iter()
                .cloned()
                .map(|polygon| PyPolygon(polygon).__str__(py))
                .collect::<pyo3::PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Difference::difference(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                ),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PyMultisegment {
    #[new]
    #[pyo3(signature = (segments, /))]
    fn new(segments: &pyo3::types::PySequence) -> pyo3::PyResult<Self> {
        super::packing::try_pack_segments(
            super::conversion::extract_from_py_sequence::<Segment, PySegment>(
                segments,
            )?,
        )
    }

    #[getter]
    fn bounding_box(&self) -> Box {
        crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
    }

    #[getter]
    fn segments(slf: pyo3::PyRef<Self>) -> PyMultisegmentSegments {
        let segments_count =
            crate::traits::Lengthsome::len(&(&slf.0).segments());
        PyMultisegmentSegments {
            multisegment: PyMultisegmentReference::from_py_ref(slf),
            start: 0isize,
            stop: segments_count as isize,
            step: 1isize,
        }
    }

    fn is_valid(&self) -> bool {
        crate::bentley_ottmann::is_multisegment_valid(&self.0)
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyContour>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<Self>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<Self>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PySegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PySegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyPolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultipolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultipolygon>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyFrozenSet::new(py, (&self.0).segments())?.hash()
    }

    fn __or__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}({})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            pyo3::IntoPy::into_py(
                crate::traits::Iterable::iter(&(&self.0).segments())
                    .cloned()
                    .collect::<Vec<_>>(),
                py
            )
            .as_ref(py)
            .repr()?
            .extract::<String>()?
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}([{}])",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            crate::traits::Iterable::iter(&(&self.0).segments())
                .cloned()
                .map(|segment| PySegment(segment).__str__(py))
                .collect::<pyo3::PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Difference::difference(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PyPoint {
    #[new]
    #[pyo3(signature = (x, y, /))]
    fn new(x: &pyo3::PyAny, y: &pyo3::PyAny) -> pyo3::prelude::PyResult<Self> {
        Ok(Self(Point::new(
            TryFromPyAny::try_from_py_any(x)?,
            TryFromPyAny::try_from_py_any(y)?,
        )))
    }

    #[getter]
    fn x<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        crate::traits::Elemental::x(&self.0).try_to_py_any(py)
    }

    #[getter]
    fn y<'a>(
        &self,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        crate::traits::Elemental::y(&self.0).try_to_py_any(py)
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyTuple::new(py, [self.x(py)?, self.y(py)?]).hash()
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.x(py)?.repr()?.extract::<String>()?,
            self.y(py)?.repr()?.extract::<String>()?,
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ge => {
                    Ok(pyo3::IntoPy::into_py(self.0 >= other.0, py))
                }
                pyo3::basic::CompareOp::Gt => {
                    Ok(pyo3::IntoPy::into_py(self.0 > other.0, py))
                }
                pyo3::basic::CompareOp::Le => {
                    Ok(pyo3::IntoPy::into_py(self.0 <= other.0, py))
                }
                pyo3::basic::CompareOp::Lt => {
                    Ok(pyo3::IntoPy::into_py(self.0 < other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.x(py)?.str()?.extract::<String>()?,
            self.y(py)?.str()?.extract::<String>()?,
        ))
    }
}

#[pyo3::prelude::pymethods]
impl PyPolygon {
    #[new]
    #[pyo3(signature = (border, holes, /))]
    fn new(
        border: &PyContour,
        holes: &pyo3::types::PySequence,
    ) -> pyo3::PyResult<Self> {
        Ok(PyPolygon(Polygon::new(
            border.0.clone(),
            super::conversion::extract_from_py_sequence::<Contour, PyContour>(
                holes,
            )?,
        )))
    }

    #[getter]
    fn border(&self) -> Contour {
        (&self.0).border().clone()
    }

    #[getter]
    fn bounding_box(&self) -> Box {
        crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
    }

    #[getter]
    fn holes(slf: pyo3::PyRef<Self>) -> PyPolygonHoles {
        let holes_count = crate::traits::Lengthsome::len(&(&slf.0).holes());
        PyPolygonHoles {
            polygon: PyPolygonReference::from_py_ref(slf),
            start: 0isize,
            stop: holes_count as isize,
            step: 1isize,
        }
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<Self>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<Self>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyContour>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultipolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultipolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultisegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PySegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PySegment>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    fn __and__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyTuple::new(
            py,
            &[
                pyo3::IntoPy::into_py(self.border(), py),
                pyo3::IntoPy::into_py(
                    pyo3::types::PyFrozenSet::new(py, (&self.0).holes())?,
                    py,
                ),
            ],
        )
        .hash()
    }

    fn __or__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Union::union(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            PyContour(self.border()).__repr__(py)?,
            pyo3::IntoPy::into_py(
                crate::traits::Iterable::iter(&(&self.0).holes())
                    .cloned()
                    .collect::<Vec<_>>(),
                py
            )
            .as_ref(py)
            .repr()?
            .extract::<String>()?
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
        Ok(format!(
            "{}({}, [{}])",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            PyContour(self.border()).__str__(py)?,
            crate::traits::Iterable::iter(&(&self.0).holes())
                .cloned()
                .map(|hole| PyContour(hole).__str__(py))
                .collect::<pyo3::PyResult<Vec<String>>>()?
                .join(", ")
        ))
    }

    fn __sub__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Difference::difference(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __xor__(&self, other: &pyo3::PyAny) -> pyo3::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                ),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PySegment {
    #[new]
    #[pyo3(signature = (start, end, /))]
    fn new(start: &PyPoint, end: &PyPoint) -> Self {
        PySegment(Segment::new(start.0.clone(), end.0.clone()))
    }

    #[getter]
    fn bounding_box(&self) -> Box {
        crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
    }

    #[getter]
    fn end(&self) -> PyPoint {
        PyPoint(crate::traits::Segmental::end(&self.0).clone())
    }

    #[getter]
    fn start(&self) -> PyPoint {
        PyPoint(crate::traits::Segmental::start(&self.0).clone())
    }

    #[pyo3(signature = (other, /))]
    fn relate_to<'a>(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        if other.is_instance_of::<PyContour>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyEmpty>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultisegment>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<Self>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<Self>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyPolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
                ),
                py,
            )
        } else if other.is_instance_of::<PyMultipolygon>() {
            TryToPyAny::try_to_py_any(
                crate::relatable::Relatable::relate_to(
                    &self.0,
                    &other.extract::<pyo3::PyRef<PyMultipolygon>>()?.0,
                ),
                py,
            )
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "Expected compound geometry, but got {}.",
                other.get_type().repr()?
            )))
        }
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    fn __and__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                PyEmpty(crate::traits::Intersection::intersection(
                    &self.0, &other.0,
                )),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            match crate::traits::Intersection::intersection(&self.0, &other.0)
            {
                None => Ok(pyo3::IntoPy::into_py(PyEmpty::new(), py)),
                Some(segment) => Ok(pyo3::IntoPy::into_py(segment, py)),
            }
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }

    fn __hash__(
        &self,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::ffi::Py_hash_t> {
        pyo3::types::PyFrozenSet::new(
            py,
            &[
                pyo3::IntoPy::into_py(self.start(), py),
                pyo3::IntoPy::into_py(self.end(), py),
            ],
        )?
        .hash()
    }

    fn __or__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __repr__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.start().__repr__(py)?,
            self.end().__repr__(py)?,
        ))
    }

    fn __richcmp__(
        &self,
        other: &pyo3::PyAny,
        op: pyo3::basic::CompareOp,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        let py = other.py();
        if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            match op {
                pyo3::basic::CompareOp::Eq => {
                    Ok(pyo3::IntoPy::into_py(self.0 == other.0, py))
                }
                pyo3::basic::CompareOp::Ne => {
                    Ok(pyo3::IntoPy::into_py(self.0 != other.0, py))
                }
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __sub__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Difference::difference(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: pyo3::Python) -> pyo3::prelude::PyResult<String> {
        Ok(format!(
            "{}({}, {})",
            <Self as pyo3::type_object::PyTypeInfo>::NAME,
            self.start().__str__(py)?,
            self.end().__str__(py)?,
        ))
    }

    fn __xor__(
        &self,
        other: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
        if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    ),
                ),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<
                PyEmpty,
                PyMultisegment,
                Segment,
            >(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
    }
}

#[pyo3::prelude::pymethods]
impl PyTrapezoidation {
    #[classmethod]
    #[pyo3(signature = (multisegment, seed, /))]
    fn from_multisegment(
        _: &pyo3::types::PyType,
        multisegment: &PyMultisegment,
        seed: usize,
    ) -> Self {
        PyTrapezoidation(Trapezoidation::from_multisegment(
            &multisegment.0,
            |values| crate::operations::permute(values, seed),
        ))
    }

    #[classmethod]
    #[pyo3(signature = (polygon, seed, /))]
    fn from_polygon(
        _: &pyo3::types::PyType,
        polygon: &PyPolygon,
        seed: usize,
    ) -> Self {
        PyTrapezoidation(Trapezoidation::from_polygon(&polygon.0, |values| {
            crate::operations::permute(values, seed);
        }))
    }

    #[getter]
    fn height(&self) -> usize {
        self.0.height()
    }

    #[pyo3(signature = (point, /))]
    fn locate<'a>(
        &self,
        point: &PyPoint,
        py: pyo3::Python<'a>,
    ) -> pyo3::PyResult<&'a pyo3::PyAny> {
        TryToPyAny::try_to_py_any(
            crate::locatable::Locatable::locate(&self.0, &point.0),
            py,
        )
    }

    fn __contains__(&self, point: &PyPoint) -> bool {
        crate::locatable::Locatable::locate(&self.0, &point.0)
            != crate::locatable::Location::Exterior
    }
}

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
