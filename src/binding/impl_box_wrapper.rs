macro_rules! impl_box_wrapper {
    () => {
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

            fn __repr__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<String> {
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

            fn __str__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<String> {
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
    };
}

pub(super) use impl_box_wrapper;
