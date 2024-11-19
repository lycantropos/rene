macro_rules! impl_box_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyBox {
            #[new]
            #[pyo3(signature = (min_x, max_x, min_y, max_y, /))]
            fn new(
                min_x: &pyo3::Bound<'_, pyo3::PyAny>,
                max_x: &pyo3::Bound<'_, pyo3::PyAny>,
                min_y: &pyo3::Bound<'_, pyo3::PyAny>,
                max_y: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<Self> {
                Ok(Self(Box::new(
                    TryFromPyAny::try_from_py_any(min_x, py)?,
                    TryFromPyAny::try_from_py_any(max_x, py)?,
                    TryFromPyAny::try_from_py_any(min_y, py)?,
                    TryFromPyAny::try_from_py_any(max_y, py)?,
                )))
            }

            #[getter]
            fn max_x<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    self.0.get_max_x(),
                    py,
                )
            }

            #[getter]
            fn max_y<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    self.0.get_max_y(),
                    py,
                )
            }

            #[getter]
            fn min_x<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    self.0.get_min_x(),
                    py,
                )
            }

            #[getter]
            fn min_y<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    self.0.get_min_y(),
                    py,
                )
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
            fn relate_to<'py>(
                &self,
                other: &Self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
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

            fn __hash__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<isize> {
                pyo3::types::PyAnyMethods::hash(
                    pyo3::types::PyTuple::new(
                        py,
                        [
                            self.min_x(py)?,
                            self.max_x(py)?,
                            self.min_y(py)?,
                            self.max_y(py)?,
                        ],
                    )?
                    .as_ref(),
                )
            }

            fn __repr__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<String> {
                use pyo3::types::PyAnyMethods;
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    match op {
                        pyo3::basic::CompareOp::Eq => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(
                                    self.0 == other.0,
                                    py,
                                )
                                .unwrap(),
                            )
                            .into_any()
                            .unbind())
                        }
                        pyo3::basic::CompareOp::Ne => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(
                                    self.0 != other.0,
                                    py,
                                )
                                .unwrap(),
                            )
                            .into_any()
                            .unbind())
                        }
                        _ => Ok(py.NotImplemented()),
                    }
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __str__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                use pyo3::types::PyAnyMethods;
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
