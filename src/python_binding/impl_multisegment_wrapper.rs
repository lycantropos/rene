macro_rules! impl_multisegment_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyMultisegment {
            #[new]
            #[pyo3(signature = (segments, /))]
            fn new(
                segments: &pyo3::Bound<'_, pyo3::types::PySequence>,
            ) -> pyo3::PyResult<Self> {
                super::packing::try_pack_segments(
                    super::conversion::extract_from_py_sequence::<
                        Segment,
                        PySegment,
                    >(segments)?,
                )
            }

            #[getter]
            fn bounding_box(&self) -> Box {
                crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
            }

            #[getter]
            fn segments(slf: pyo3::PyRef<'_, Self>) -> PyMultisegmentSegments {
                let segments_count = crate::traits::Lengthsome::len(
                    &crate::traits::Multisegmental::segments(&slf.0),
                );
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
            fn locate<'py>(
                &self,
                point: &pyo3::Bound<'_, PyPoint>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                TryToPyAny::try_to_py_any(
                    crate::locatable::Locatable::locate(
                        &self.0,
                        &point.borrow().0,
                    ),
                    py,
                )
            }

            #[pyo3(signature = (other, /))]
            fn relate_to<'py>(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance_of::<PyEmpty>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, PyEmpty>>()?
                                .borrow()
                                .0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyContour>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, PyContour>>()?
                                .borrow()
                                .0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<Self>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, Self>>()?
                                .borrow()
                                .0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PySegment>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, PySegment>>()?
                                .borrow()
                                .0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyPolygon>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, PyPolygon>>()?
                                .borrow()
                                .0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyMultipolygon>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other
                                .extract::<pyo3::Bound<'_, PyMultipolygon>>()?
                                .borrow()
                                .0,
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
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    pyo3::IntoPyObject::into_pyobject(
                        PyEmpty(crate::traits::Intersection::intersection(
                            &self.0, &other.0,
                        )),
                        py,
                    )
                    .map(pyo3::Bound::into_any)
                    .map(pyo3::Bound::unbind)
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __contains__(&self, point: &pyo3::Bound<'_, PyPoint>) -> bool {
                crate::locatable::Locatable::locate(&self.0, &point.borrow().0)
                    != crate::locatable::Location::Exterior
            }

            fn __hash__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<isize> {
                pyo3::types::PyAnyMethods::hash(
                    pyo3::types::PyFrozenSet::new(
                        py,
                        crate::traits::Multisegmental::segments(&self.0)
                            .into_iter()
                            .cloned(),
                    )?
                    .as_ref(),
                )
            }

            fn __or__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    pyo3::IntoPyObject::into_pyobject(
                        Self(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    )
                    .map(pyo3::Bound::into_any)
                    .map(pyo3::Bound::unbind)
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other
                        .extract::<pyo3::Bound<'_, PyContour>>()?
                        .borrow();
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    super::unpacking::try_unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    super::unpacking::try_unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other
                        .extract::<pyo3::Bound<'_, PySegment>>()?
                        .borrow();
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    super::unpacking::try_unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __repr__(
                &self,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<String> {
                use pyo3::types::PyAnyMethods;
                Ok(format!(
                    "{}({})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    pyo3::IntoPyObject::into_pyobject(
                        crate::traits::Iterable::iter(
                            &crate::traits::Multisegmental::segments(&self.0)
                        )
                        .cloned()
                        .collect::<Vec<_>>(),
                        py
                    )?
                    .repr()?
                    .extract::<String>()?
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
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    match op {
                        pyo3::basic::CompareOp::Eq => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(self.0 == other.0, py)
                                    .unwrap()
                            )
                                .into_any()
                                .unbind())
                        }
                        pyo3::basic::CompareOp::Ne => {
                            Ok(pyo3::BoundObject::into_bound(
                                pyo3::IntoPyObject::into_pyobject(self.0 != other.0, py)
                                    .unwrap()
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
                Ok(format!(
                    "{}([{}])",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    crate::traits::Iterable::iter(
                        &crate::traits::Multisegmental::segments(&self.0)
                    )
                    .cloned()
                    .map(|segment| PySegment(segment).__str__(py))
                    .collect::<pyo3::PyResult<Vec<String>>>()?
                    .join(", ")
                ))
            }

            fn __sub__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    pyo3::IntoPyObject::into_pyobject(
                        Self(crate::traits::Difference::difference(&self.0, &other.0)),
                        py,
                    )
                    .map(pyo3::Bound::into_any)
                    .map(pyo3::Bound::unbind)
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    super::unpacking::try_unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment, _>(
                        segments,
                        py,
                    )
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __xor__(
                &self,
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                py: pyo3::Python<'_>,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                if other.is_instance(
                    &<PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, PyEmpty>>()?.borrow();
                    pyo3::IntoPyObject::into_pyobject(
                        Self(
                            crate::traits::SymmetricDifference::symmetric_difference(
                                &self.0, &other.0,
                            ),
                        ),
                        py,
                    )
                    .map(pyo3::Bound::into_any)
                    .map(pyo3::Bound::unbind)
                } else if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other
                        .extract::<pyo3::Bound<'_, PyContour>>()?
                        .borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    super::unpacking::try_unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else if other.is_instance(
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other =
                        other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    super::unpacking::try_unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other
                        .extract::<pyo3::Bound<'_, PySegment>>()?
                        .borrow();
                    let segments =
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        );
                    super::unpacking::try_unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                        _,
                    >(segments, py)
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

pub(super) use impl_multisegment_wrapper;
