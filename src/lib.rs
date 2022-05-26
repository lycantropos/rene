use std::cmp::Ordering;
use std::convert::TryFrom;

use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyOverflowError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};
use pyo3::type_object::PyTypeObject;
use pyo3::types::{PyFloat, PyLong, PySequence, PyTuple};
use pyo3::{ffi, intern, AsPyPointer, IntoPy, Py, PyAny, PyErr, PyObject, ToPyObject};
use rithm::traits::{Endianness, FromBytes, ToBytes, Zeroable};
use rithm::{big_int, fraction};

use crate::geometries::MIN_CONTOUR_VERTICES_COUNT;
use crate::oriented::{Orientation, Oriented};
use crate::traits::{Contour, Point, Polygon, Segment};

pub mod geometries;
pub mod oriented;
pub mod traits;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (Digit::BITS - 1) as usize;

type BigInt = big_int::BigInt<Digit, '_', BINARY_SHIFT>;
type Fraction = fraction::Fraction<BigInt>;
type ExactContour = geometries::Contour<Fraction>;
type ExactPoint = geometries::Point<Fraction>;
type ExactPolygon = geometries::Polygon<Fraction>;
type ExactSegment = geometries::Segment<Fraction>;

impl IntoPy<PyObject> for ExactContour {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactContour(self).into_py(py)
    }
}

impl ToPyObject for ExactPoint {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        self.clone().into_py(py)
    }
}

impl IntoPy<PyObject> for ExactPoint {
    fn into_py(self, py: Python<'_>) -> PyObject {
        PyExactPoint(self).into_py(py)
    }
}

#[pyclass(name = "Orientation", module = "rene")]
#[derive(Clone)]
struct PyOrientation(Orientation);

#[pymethods]
impl PyOrientation {
    #[classattr]
    const CLOCKWISE: PyOrientation = PyOrientation(Orientation::Clockwise);

    #[classattr]
    const COLLINEAR: PyOrientation = PyOrientation(Orientation::Collinear);

    #[classattr]
    const COUNTERCLOCKWISE: PyOrientation = PyOrientation(Orientation::Counterclockwise);

    #[getter]
    fn value(&self) -> i8 {
        match self.0 {
            Orientation::Clockwise => -1,
            Orientation::Collinear => 0,
            Orientation::Counterclockwise => 1,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "rene.Orientation.{}",
            match self.0 {
                Orientation::Clockwise => "CLOCKWISE",
                Orientation::Collinear => "COLLINEAR",
                Orientation::Counterclockwise => "COUNTERCLOCKWISE",
            }
        )
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyOrientation::type_object(py))? {
            let other = other.extract::<PyOrientation>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self) -> String {
        format!(
            "Orientation.{}",
            match self.0 {
                Orientation::Clockwise => "CLOCKWISE",
                Orientation::Collinear => "COLLINEAR",
                Orientation::Counterclockwise => "COUNTERCLOCKWISE",
            }
        )
    }
}

#[pyclass(name = "Contour", module = "rene.exact", subclass)]
#[pyo3(text_signature = "(vertices, /)")]
#[derive(Clone)]
struct PyExactContour(ExactContour);

#[pyclass(name = "Point", module = "rene.exact", subclass)]
#[pyo3(text_signature = "(x, y, /)")]
#[derive(Clone)]
struct PyExactPoint(ExactPoint);

#[pyclass(name = "Polygon", module = "rene.exact", subclass)]
#[pyo3(text_signature = "(border, holes, /)")]
#[derive(Clone)]
struct PyExactPolygon(ExactPolygon);

#[pyclass(name = "Segment", module = "rene.exact", subclass)]
#[pyo3(text_signature = "(start, end, /)")]
#[derive(Clone)]
struct PyExactSegment(ExactSegment);

#[pymethods]
impl PyExactContour {
    #[new]
    fn new(vertices: &PySequence) -> PyResult<Self> {
        let mut result_vertices = Vec::<ExactPoint>::with_capacity(vertices.len()?);
        for vertex in vertices.iter()? {
            result_vertices.push(vertex?.extract::<PyExactPoint>()?.0);
        }
        if result_vertices.len() < MIN_CONTOUR_VERTICES_COUNT {
            Err(PyValueError::new_err(format!(
                "Contour should have at least {} vertices, but found {}.",
                MIN_CONTOUR_VERTICES_COUNT,
                result_vertices.len()
            )))
        } else {
            Ok(PyExactContour(ExactContour::new(result_vertices)))
        }
    }

    #[getter]
    fn vertices(&self) -> Vec<ExactPoint> {
        self.0.vertices()
    }

    #[getter]
    fn orientation(&self, py: Python) -> PyResult<&PyAny> {
        let orientation = self.0.orientation();
        let orientation_cls =
            unsafe { ROOT_MODULE.unwrap_unchecked() }.getattr(intern!(py, "Orientation"))?;
        match orientation {
            Orientation::Clockwise => orientation_cls.getattr(intern!(py, "CLOCKWISE")),
            Orientation::Collinear => orientation_cls.getattr(intern!(py, "COLLINEAR")),
            Orientation::Counterclockwise => {
                orientation_cls.getattr(intern!(py, "COUNTERCLOCKWISE"))
            }
        }
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        let mut vertices = self.0.vertices();
        vertices.rotate_left(self.0.to_min_vertex_index());
        match self.0.orientation() {
            Orientation::Clockwise => vertices[1..].reverse(),
            _ => {}
        }
        PyTuple::new(py, &vertices).hash()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Contour({})",
            self.vertices()
                .into_py(py)
                .as_ref(py)
                .repr()?
                .extract::<String>()?
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactContour::type_object(py))? {
            let other = other.extract::<PyExactContour>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "Contour([{}])",
            self.vertices()
                .into_iter()
                .flat_map(|vertex| PyExactPoint(vertex).__str__(py))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}

#[pymethods]
impl PyExactPoint {
    #[new]
    fn new(x: &PyAny, y: &PyAny) -> PyResult<Self> {
        Ok(PyExactPoint(ExactPoint::new(
            try_scalar_to_fraction(x)?,
            try_scalar_to_fraction(y)?,
        )))
    }

    #[getter]
    fn x<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        try_fraction_to_py_fraction(self.0.x(), py)
    }

    #[getter]
    fn y<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        try_fraction_to_py_fraction(self.0.y(), py)
    }

    fn __hash__(&self, py: Python) -> PyResult<ffi::Py_hash_t> {
        PyTuple::new(py, [self.x(py)?, self.y(py)?]).hash()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Point({}, {})",
            self.x(py)?.repr()?.extract::<String>()?,
            self.y(py)?.repr()?.extract::<String>()?,
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactPoint::type_object(py))? {
            let other = other.extract::<PyExactPoint>()?;
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

    fn __str__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "Point({}, {})",
            self.x(py)?.str()?.extract::<String>()?,
            self.y(py)?.str()?.extract::<String>()?,
        ))
    }
}

#[pymethods]
impl PyExactPolygon {
    #[new]
    fn new(border: &PyExactContour, holes: &PySequence) -> PyResult<Self> {
        let mut result_holes = Vec::<ExactContour>::with_capacity(holes.len()?);
        for hole in holes.iter()? {
            result_holes.push(hole?.extract::<PyExactContour>()?.0);
        }
        Ok(PyExactPolygon(ExactPolygon::new(
            border.0.clone(),
            result_holes,
        )))
    }

    #[getter]
    fn border(&self) -> ExactContour {
        self.0.border()
    }

    #[getter]
    fn holes(&self) -> Vec<ExactContour> {
        self.0.holes()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Polygon({}, {})",
            PyExactContour(self.border()).__repr__(py)?,
            self.holes()
                .into_py(py)
                .as_ref(py)
                .repr()?
                .extract::<String>()?
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactPolygon::type_object(py))? {
            let other = other.extract::<PyExactPolygon>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "Polygon({}, [{}])",
            PyExactContour(self.border()).__str__(py)?,
            self.holes()
                .into_iter()
                .flat_map(|hole| PyExactContour(hole).__str__(py))
                .collect::<Vec<String>>()
                .join(", ")
        ))
    }
}

#[pymethods]
impl PyExactSegment {
    #[new]
    fn new(start: &PyExactPoint, end: &PyExactPoint) -> PyResult<Self> {
        Ok(PyExactSegment(ExactSegment::new(
            start.0.clone(),
            end.0.clone(),
        )))
    }

    #[getter]
    fn start(&self) -> PyExactPoint {
        PyExactPoint(self.0.start())
    }

    #[getter]
    fn end(&self) -> PyExactPoint {
        PyExactPoint(self.0.end())
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Segment({}, {})",
            self.start().__repr__(py)?,
            self.end().__repr__(py)?,
        ))
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<PyObject> {
        let py = other.py();
        if other.is_instance(PyExactSegment::type_object(py))? {
            let other = other.extract::<PyExactSegment>()?;
            match op {
                CompareOp::Eq => Ok((self.0 == other.0).into_py(py)),
                CompareOp::Ne => Ok((self.0 != other.0).into_py(py)),
                _ => Ok(py.NotImplemented()),
            }
        } else {
            Ok(py.NotImplemented())
        }
    }

    fn __str__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Segment({}, {})",
            self.start().__str__(py)?,
            self.end().__str__(py)?,
        ))
    }
}

fn try_fraction_to_py_fraction(value: Fraction, py: Python) -> PyResult<&PyAny> {
    let rithm_module = py.import("rithm")?;
    let fraction_cls = rithm_module.getattr("Fraction")?;
    fraction_cls.call(
        (
            big_int_to_py_long(value.numerator()),
            big_int_to_py_long(value.denominator()),
        ),
        None,
    )
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
    let ptr = value
        .call_method0("__int__")
        .or(Err(PyTypeError::new_err(INVALID_SCALAR_TYPE_ERROR_MESSAGE)))?
        .as_ptr();
    let py = value.py();
    unsafe {
        let ptr = ffi::PyNumber_Index(ptr);
        if ptr.is_null() {
            return Err(PyErr::fetch(py));
        }
        let bits_count = ffi::_PyLong_NumBits(ptr);
        match bits_count.cmp(&0) {
            Ordering::Less => Err(PyErr::fetch(py)),
            Ordering::Equal => Ok(BigInt::zero()),
            Ordering::Greater => {
                let bytes_count = (bits_count as usize) / (u8::BITS as usize) + 1;
                let mut buffer = vec![0u8; bytes_count];
                if ffi::_PyLong_AsByteArray(
                    Py::<PyLong>::from_owned_ptr(py, ptr).as_ptr() as *mut ffi::PyLongObject,
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

const INVALID_SCALAR_TYPE_ERROR_MESSAGE: &str = "Scalar should be a rational number.";
const UNDEFINED_DIVISION_ERROR_MESSAGE: &str = "Division by zero is undefined.";

fn try_scalar_to_fraction(value: &PyAny) -> PyResult<Fraction> {
    let py = value.py();
    if value.is_instance(PyFloat::type_object(py))? {
        Fraction::try_from(value.extract::<f64>()?).map_err(|reason| match reason {
            fraction::FromFloatConversionError::Infinity => {
                PyOverflowError::new_err(reason.to_string())
            }
            _ => PyValueError::new_err(reason.to_string()),
        })
    } else {
        let numerator = try_py_integral_to_big_int(
            value
                .getattr(intern!(py, "numerator"))
                .or(Err(PyTypeError::new_err(INVALID_SCALAR_TYPE_ERROR_MESSAGE)))?,
        )?;
        let denominator = try_py_integral_to_big_int(
            value
                .getattr(intern!(py, "denominator"))
                .or(Err(PyTypeError::new_err(INVALID_SCALAR_TYPE_ERROR_MESSAGE)))?,
        )?;
        match Fraction::new(numerator, denominator) {
            Some(value) => Ok(value),
            None => Err(PyZeroDivisionError::new_err(
                UNDEFINED_DIVISION_ERROR_MESSAGE,
            )),
        }
    }
}

static mut ROOT_MODULE: Option<&PyModule> = None;

#[pymodule]
fn _exact(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyExactContour>()?;
    module.add_class::<PyExactPoint>()?;
    module.add_class::<PyExactPolygon>()?;
    module.add_class::<PyExactSegment>()?;
    unsafe {
        let py = Python::assume_gil_acquired();
        ROOT_MODULE = Some(py.import("rene")?);
    }
    Ok(())
}

#[pymodule]
fn _rene(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyOrientation>()?;
    module.add("MIN_CONTOUR_VERTICES_COUNT", MIN_CONTOUR_VERTICES_COUNT)?;
    Ok(())
}
