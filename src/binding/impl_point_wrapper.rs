macro_rules! impl_point_wrapper {
    () => {
        #[pyo3::prelude::pymethods]
        impl PyPoint {
            #[new]
            #[pyo3(signature = (x, y, /))]
            fn new(
                x: &pyo3::PyAny,
                y: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<Self> {
                Ok(Self(Point::new(
                    TryFromPyAny::try_from_py_any(x, py)?,
                    TryFromPyAny::try_from_py_any(y, py)?,
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
                pyo3::types::PyTuple::new(py, [self.x(py)?, self.y(py)?])
                    .hash()
            }

            fn __repr__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<String> {
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

            fn __str__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<String> {
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    self.x(py)?.str()?.extract::<String>()?,
                    self.y(py)?.str()?.extract::<String>()?,
                ))
            }
        }
    };
}

pub(super) use impl_point_wrapper;
