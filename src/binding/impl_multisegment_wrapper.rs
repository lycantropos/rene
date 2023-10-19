macro_rules! impl_multisegment_wrapper {
    () => {
        #[pyo3::prelude::pymethods]
        impl PyMultisegment {
            #[new]
            #[pyo3(signature = (segments, /))]
            fn new(
                segments: &pyo3::types::PySequence,
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
            fn segments(slf: pyo3::PyRef<Self>) -> PyMultisegmentSegments {
                let segments_count =
                    crate::traits::Lengthsome::len(&crate::traits::Multisegmental::segments(&slf.0));
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
            fn locate<'a>(
                &self,
                point: &PyPoint,
                py: pyo3::Python<'a>,
            ) -> pyo3::PyResult<&'a pyo3::PyAny> {
                TryToPyAny::try_to_py_any(
                    crate::locatable::Locatable::locate(&self.0, &point.0),
                    py,
                )
            }

            #[pyo3(signature = (other, /))]
            fn relate_to<'a>(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python<'a>,
            ) -> pyo3::PyResult<&'a pyo3::PyAny> {
                if other.is_instance_of::<PyEmpty>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<PyEmpty>>()?.0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<PyContour>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<PyContour>>()?.0,
                        ),
                        py,
                    )
                } else if other.is_instance_of::<Self>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<Self>>()?.0,
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
                } else if other.is_instance_of::<PyPolygon>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<PyPolygon>>()?.0,
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
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if other.is_instance(
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
                <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyContour>>()?;
                let segments =
                    crate::traits::Intersection::intersection(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<Self>>()?;
                let segments =
                    crate::traits::Intersection::intersection(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PySegment>>()?;
                let segments =
                    crate::traits::Intersection::intersection(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
                let segments =
                    crate::traits::Intersection::intersection(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
                let segments =
                    crate::traits::Intersection::intersection(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else {
                Ok(py.NotImplemented())
            }
            }

            fn __contains__(&self, point: &PyPoint) -> bool {
                crate::locatable::Locatable::locate(&self.0, &point.0)
                    != crate::locatable::Location::Exterior
            }

            fn __hash__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::ffi::Py_hash_t> {
                pyo3::types::PyFrozenSet::new(py, crate::traits::Multisegmental::segments(&self.0))?.hash()
            }

            fn __or__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if other.is_instance(
                    <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
                    Ok(pyo3::IntoPy::into_py(
                        Self(crate::traits::Union::union(&self.0, &other.0)),
                        py,
                    ))
                } else if other.is_instance(
                    <PyContour as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyContour>>()?;
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::PyRef<Self>>()?;
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    <PySegment as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PySegment>>()?;
                    let segments =
                        crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
                Ok(format!(
                    "{}({})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    pyo3::IntoPy::into_py(
                        crate::traits::Iterable::iter(&crate::traits::Multisegmental::segments(&self.0))
                            .cloned()
                            .collect::<Vec<_>>(),
                        py
                    )
                    .as_ref(py)
                    .repr()?
                    .extract::<String>()?
                ))
            }

            fn __richcmp__(
                &self,
                other: &pyo3::PyAny,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(
                <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
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

            fn __str__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
                Ok(format!(
                    "{}([{}])",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    crate::traits::Iterable::iter(&crate::traits::Multisegmental::segments(&self.0))
                        .cloned()
                        .map(|segment| PySegment(segment).__str__(py))
                        .collect::<pyo3::PyResult<Vec<String>>>()?
                        .join(", ")
                ))
            }

            fn __sub__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if other.is_instance(
                <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
                Ok(pyo3::IntoPy::into_py(
                    Self(crate::traits::Difference::difference(&self.0, &other.0)),
                    py,
                ))
            } else if other.is_instance(
                <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyContour>>()?;
                let segments =
                    crate::traits::Difference::difference(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<Self>>()?;
                let segments =
                    crate::traits::Difference::difference(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PySegment>>()?;
                let segments =
                    crate::traits::Difference::difference(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
                let segments =
                    crate::traits::Difference::difference(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else if other.is_instance(
                <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
            )? {
                let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
                let segments =
                    crate::traits::Difference::difference(&self.0, &other.0);
                Ok(super::unpacking::unpack_maybe_empty_segments::<
                    PyEmpty,
                    PyMultisegment,
                    Segment,
                >(segments, py))
            } else {
                Ok(py.NotImplemented())
            }
            }

            fn __xor__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if other.is_instance(
                    <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
                    Ok(pyo3::IntoPy::into_py(
                    Self(
                        crate::traits::SymmetricDifference::symmetric_difference(
                            &self.0, &other.0,
                        ),
                    ),
                    py,
                ))
                } else if other.is_instance(
                    <PyContour as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyContour>>()?;
                    let segments =
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
                )? {
                    let other = other.extract::<pyo3::PyRef<Self>>()?;
                    let segments =
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    <PySegment as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PySegment>>()?;
                    let segments =
                    crate::traits::SymmetricDifference::symmetric_difference(
                        &self.0, &other.0,
                    );
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }
        }
    };
}

pub(super) use impl_multisegment_wrapper;
