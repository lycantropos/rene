macro_rules! impl_contour_wrapper {
    () => {
        #[pyo3::prelude::pymethods]
        impl PyContour {
            #[new]
            #[pyo3(signature = (vertices, /))]
            fn new(
                vertices: &pyo3::types::PySequence,
            ) -> pyo3::PyResult<Self> {
                super::packing::try_pack_vertices(
                    super::conversion::extract_from_py_sequence::<
                        Point,
                        PyPoint,
                    >(vertices)?,
                )
            }

            #[getter]
            fn bounding_box(&self) -> Box {
                crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
            }

            #[getter]
            fn segments(slf: pyo3::PyRef<Self>) -> PyContourSegments {
                let segments_count =
                    crate::traits::Lengthsome::len(&(&slf.0).segments());
                PyContourSegments {
                    contour: PyContourReference::from_py_ref(slf),
                    start: 0isize,
                    stop: segments_count as isize,
                    step: 1isize,
                }
            }

            #[getter]
            fn vertices(slf: pyo3::PyRef<Self>) -> PyContourVertices {
                let vertices_count =
                    crate::traits::Lengthsome::len(&(&slf.0).vertices());
                PyContourVertices {
                    contour: PyContourReference::from_py_ref(slf),
                    start: 0isize,
                    stop: vertices_count as isize,
                    step: 1isize,
                }
            }

            #[getter]
            fn orientation<'a>(
                &self,
                py: pyo3::Python<'a>,
            ) -> pyo3::PyResult<&'a pyo3::PyAny> {
                crate::oriented::Oriented::to_orientation(&self.0)
                    .try_to_py_any(py)
            }

            fn is_valid(&self) -> bool {
                crate::bentley_ottmann::is_contour_valid(&self.0)
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
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
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
                let mut vertices =
                    (&self.0).vertices().into_iter().collect::<Vec<_>>();
                let min_vertex_index = unsafe {
                    crate::operations::to_arg_min(&vertices).unwrap_unchecked()
                };
                vertices.rotate_left(min_vertex_index);
                if crate::oriented::Oriented::to_orientation(&self.0)
                    == crate::oriented::Orientation::Clockwise
                {
                    vertices[1..].reverse();
                }
                pyo3::types::PyTuple::new(py, &vertices).hash()
            }

            fn __or__(
                &self,
                other: &pyo3::PyAny,
                py: pyo3::Python,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                Self(crate::traits::Union::union(&self.0, &other.0)),
                py,
            ))
        } else if other.is_instance(
            <Self as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<Self>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_segments::<
                PyMultisegment,
                Segment,
            >(segments, py))
        } else if other.is_instance(
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PySegment>>()?;
            let segments = crate::traits::Union::union(&self.0, &other.0);
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
                    "{}([{}])",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    (&self.0)
                        .vertices()
                        .into_iter()
                        .cloned()
                        .map(|vertex| PyPoint(vertex).__repr__(py))
                        .collect::<pyo3::PyResult<Vec<String>>>()?
                        .join(", ")
                ))
            }

            fn __richcmp__(
                &self,
                other: &pyo3::PyAny,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(
                    <PyContour as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyContour>>()?;
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
                    "Contour([{}])",
                    (&self.0)
                        .vertices()
                        .into_iter()
                        .cloned()
                        .map(|vertex| PyPoint(vertex).__str__(py))
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
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
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
            <PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultisegment>>()?;
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
            <PySegment as pyo3::type_object::PyTypeInfo>::type_object(py),
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

pub(super) use impl_contour_wrapper;
