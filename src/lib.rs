use std::cmp::Ordering;

use pyo3::exceptions::{PyOverflowError, PyTypeError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};
use pyo3::types::{PyFloat, PyLong};
use pyo3::{ffi, intern, AsPyPointer, Py, PyAny, PyErr, PyObject};
use rithm::traits::{Endianness, FromBytes, OppositionOf, ToBytes, Zeroable};
use rithm::{big_int, fraction};

use crate::traits::Point;
use pyo3::type_object::PyTypeObject;
use std::convert::TryFrom;

pub mod geometries;
pub mod traits;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;

const BINARY_SHIFT: usize = (OppositionOf::<Digit>::BITS - 2) as usize;

type BigInt = big_int::BigInt<Digit, '_', BINARY_SHIFT>;
type Fraction = fraction::Fraction<BigInt>;
type ExactPoint = geometries::Point<Fraction>;

#[pyclass(name = "Point", module = "rene", subclass)]
#[pyo3(text_signature = "(x, y, /)")]
#[derive(Clone)]
struct PyExactPoint(ExactPoint);

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

    fn __repr__(&self, py: Python) -> PyResult<String> {
        Ok(format!(
            "rene.exact.Point({}, {})",
            self.x(py)?.repr()?.extract::<String>()?,
            self.y(py)?.repr()?.extract::<String>()?,
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

#[pymodule]
fn _rene(py: Python, module: &PyModule) -> PyResult<()> {
    module.setattr(intern!(py, "__version__"), env!("CARGO_PKG_VERSION"))?;
    module.setattr(intern!(py, "__doc__"), env!("CARGO_PKG_DESCRIPTION"))?;
    Ok(())
}

#[pymodule]
fn _rene_exact(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<PyExactPoint>()?;
    Ok(())
}
