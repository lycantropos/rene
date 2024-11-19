macro_rules! impl_point_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyPoint {
            #[new]
            #[pyo3(signature = (x, y, /))]
            fn new(
                x: &pyo3::Bound<'_, pyo3::PyAny>,
                y: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<Self> {
                Ok(Self(Point::new(
                    TryFromPyAny::try_from_py_any(x, py)?,
                    TryFromPyAny::try_from_py_any(y, py)?,
                )))
            }

            #[getter]
            fn x<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    crate::traits::Elemental::x(&self.0),
                    py,
                )
            }

            #[getter]
            fn y<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
                    crate::traits::Elemental::y(&self.0),
                    py,
                )
            }

            fn __hash__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<isize> {
                pyo3::types::PyAnyMethods::hash(
                    pyo3::types::PyTuple::new(py, [self.x(py)?, self.y(py)?])?
                        .as_ref(),
                )
            }

            fn __repr__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<String> {
                use pyo3::types::PyAnyMethods;
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    self.x(py)?.repr()?.extract::<String>()?,
                    self.y(py)?.repr()?.extract::<String>()?,
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
                    Ok(pyo3::BoundObject::into_bound(
                        pyo3::IntoPyObject::into_pyobject(
                            match op {
                                pyo3::basic::CompareOp::Eq => {
                                    self.0 == other.0
                                }
                                pyo3::basic::CompareOp::Ge => {
                                    self.0 >= other.0
                                }
                                pyo3::basic::CompareOp::Gt => self.0 > other.0,
                                pyo3::basic::CompareOp::Le => {
                                    self.0 <= other.0
                                }
                                pyo3::basic::CompareOp::Lt => self.0 < other.0,
                                pyo3::basic::CompareOp::Ne => {
                                    self.0 != other.0
                                }
                            },
                            py,
                        )
                        .unwrap(),
                    )
                    .into_any()
                    .unbind())
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __str__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
                use pyo3::types::PyAnyMethods;
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
