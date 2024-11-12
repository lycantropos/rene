macro_rules! impl_segment_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PySegment {
            #[new]
            #[pyo3(signature = (start, end, /))]
            fn new(
                start: &pyo3::Bound<'_, PyPoint>,
                end: &pyo3::Bound<'_, PyPoint>,
            ) -> Self {
                PySegment(Segment::new(
                    start.borrow().0.clone(),
                    end.borrow().0.clone(),
                ))
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
                } else if other.is_instance_of::<PyMultisegment>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow().0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<Self>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, Self>>()?.borrow().0,
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
                } else if other.is_instance_of::<PyMultipolygon>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow().0,
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
            fn locate<'py>(
                &self,
                point: &pyo3::Bound<'_, PyPoint>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                 crate::python_binding::traits::TryToPyAny::try_to_py_any(
                     crate::locatable::Locatable::locate(&self.0, &point.borrow().0),
                     py,
                 )
            }

            fn __and__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
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
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    match crate::traits::Intersection::intersection(&self.0, &other.0)
                    {
                        None => Ok(pyo3::IntoPy::into_py(PyEmpty::new(), py)),
                        Some(segment) => Ok(pyo3::IntoPy::into_py(segment, py)),
                    }
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __contains__(&self, point: &pyo3::Bound<'_, PyPoint>) -> bool {
                crate::locatable::Locatable::locate(&self.0, &point.borrow().0)
                    != crate::locatable::Location::Exterior
            }

            fn __hash__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<isize> {
                pyo3::types::PyAnyMethods::hash(
                    pyo3::types::PyFrozenSet::new_bound(
                        py,
                        &[
                            pyo3::IntoPy::into_py(self.start(), py),
                            pyo3::IntoPy::into_py(self.end(), py),
                        ],
                    )?
                    .as_ref()
                )
            }

            fn __or__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        Self(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __repr__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<String> {
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    self.start().__repr__(py)?,
                    self.end().__repr__(py)?,
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
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        Self(crate::traits::Difference::difference(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __str__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<String> {
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    self.start().__str__(py)?,
                    self.end().__str__(py)?,
                ))
            }

            fn __xor__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    Ok(pyo3::IntoPy::into_py(
                        Self(
                            crate::traits::SymmetricDifference::symmetric_difference(
                                &self.0, &other.0,
                            ),
                        ),
                        py,
                    ))
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else if other.is_instance(&<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py))? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

pub(super) use impl_segment_wrapper;
