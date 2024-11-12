macro_rules! impl_empty_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyEmpty {
            #[new]
            fn new() -> Self {
                PyEmpty(Empty::new())
            }

            #[pyo3(signature = (point, /))]
            fn locate<'py>(
                &self,
                #[allow(unused_variables)] point: &pyo3::Bound<'_, PyPoint>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::locatable::Location::Exterior.try_to_py_any(py)
            }

            #[pyo3(signature = (other, /))]
            fn relate_to<'py>(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance_of::<PyContour>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyEmpty>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyMultipolygon>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyMultisegment>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyPolygon>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PySegment>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow().0,
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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

            fn __contains__(&self, _point: &pyo3::Bound<'_, PyPoint>) -> bool {
                false
            }

            fn __hash__(&self) -> isize {
                0
            }

            fn __or__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyContour(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        crate::traits::Union::union(&self.0, &other.0),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyMultisegment(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        crate::traits::Union::union(&self.0, &other.0),
                        py,
                    ))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Difference::difference(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Difference::difference(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Difference::difference(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Difference::difference(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(crate::traits::Difference::difference(
                            &self.0, &other.0,
                        )),
                        py,
                    ))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyContour(
                            crate::traits::SymmetricDifference::symmetric_difference(
                                &self.0, &other.0,
                            ),
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyEmpty(
                            crate::traits::SymmetricDifference::symmetric_difference(
                                &self.0, &other.0,
                            ),
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        PyMultisegment(
                            crate::traits::SymmetricDifference::symmetric_difference(
                                &self.0, &other.0,
                            ),
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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
