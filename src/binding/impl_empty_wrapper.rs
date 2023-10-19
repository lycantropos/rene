macro_rules! impl_empty_wrapper {
    () => {
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

            fn __and__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
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

            fn __or__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
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
                    <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
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

            fn __sub__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
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

            fn __xor__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
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
    };
}

pub(super) use impl_empty_wrapper;
