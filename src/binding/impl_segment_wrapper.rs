macro_rules! impl_segment_wrapper {
    () => {
        #[pyo3::prelude::pymethods]
        impl PySegment {
            #[new]
            #[pyo3(signature = (start, end, /))]
            fn new(start: &PyPoint, end: &PyPoint) -> Self {
                PySegment(Segment::new(start.0.clone(), end.0.clone()))
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
            fn relate_to<'a>(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python<'a>,
            ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
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
                } else if other.is_instance_of::<PyMultisegment>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<PyMultisegment>>()?.0,
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

            #[pyo3(signature = (point, /))]
            fn locate<'a>(
                &self,
                point: &PyPoint,
                py: pyo3::Python<'a>,
            ) -> pyo3::prelude::PyResult<&'a pyo3::PyAny> {
                TryToPyAny::try_to_py_any(
                    crate::locatable::Locatable::locate(&self.0, &point.0),
                    py,
                )
            }

            fn __and__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
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
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            match crate::traits::Intersection::intersection(&self.0, &other.0)
            {
                None => Ok(pyo3::IntoPy::into_py(PyEmpty::new(), py)),
                Some(segment) => Ok(pyo3::IntoPy::into_py(segment, py)),
            }
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
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
            ) -> pyo3::prelude::PyResult<pyo3::ffi::Py_hash_t> {
                pyo3::types::PyFrozenSet::new(
                    py,
                    &[
                        pyo3::IntoPy::into_py(self.start(), py),
                        pyo3::IntoPy::into_py(self.end(), py),
                    ],
                )?
                .hash()
            }

            fn __or__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<PyMultisegment, Segment>(segments, py))
        } else {
            Ok(py.NotImplemented())
        }
            }

            fn __repr__(
                &self,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<String> {
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    self.start().__repr__(py)?,
                    self.end().__repr__(py)?,
                ))
            }

            fn __richcmp__(
                &self,
                other: &pyo3::PyAny,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(
                    <PySegment as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PySegment>>()?;
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
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
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
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let segments =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
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
                    self.start().__str__(py)?,
                    self.end().__str__(py)?,
                ))
            }

            fn __xor__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::prelude::PyResult<pyo3::PyObject> {
                if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
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
            <PyContour as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyContour>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_segments::<PyEmpty, PyMultisegment, Segment>(segments, py))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
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
