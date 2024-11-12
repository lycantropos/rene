macro_rules! impl_contour_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyContour {
            #[new]
            #[pyo3(signature = (vertices, /))]
            fn new(
                vertices: &pyo3::Bound<'_, pyo3::types::PySequence>,
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
            fn segments(slf: pyo3::PyRef<'_, Self>) -> PyContourSegments {
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
            fn vertices(slf: pyo3::PyRef<'_, Self>) -> PyContourVertices {
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
            fn orientation<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::oriented::Oriented::to_orientation(&self.0)
                    .try_to_py_any(py)
            }

            fn is_valid(&self) -> bool {
                crate::bentley_ottmann::is_contour_valid(&self.0)
            }

            #[pyo3(signature = (point, /))]
            fn locate<'py>(
                &self,
                point: &pyo3::Bound<'_, PyPoint>,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
                crate::python_binding::traits::TryToPyAny::try_to_py_any(
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
                } else if other.is_instance_of::<PySegment>() {
                    crate::python_binding::traits::TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow().0,
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
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
                    let segments =
                        crate::traits::Intersection::intersection(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
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

            fn __contains__(&self, point: &pyo3::Bound<'_, PyPoint>) -> bool {
                crate::locatable::Locatable::locate(&self.0, &point.borrow().0)
                    != crate::locatable::Location::Exterior
            }

            fn __hash__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<isize> {
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
                pyo3::types::PyAnyMethods::hash(
                    pyo3::types::PyTuple::new_bound(py, &vertices).as_ref()
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
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    let segments = crate::traits::Union::union(&self.0, &other.0);
                    Ok(super::unpacking::unpack_non_empty_segments::<
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else {
                    Ok(py.NotImplemented())
                }
            }

            fn __repr__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
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
                other: &pyo3::Bound<'_, pyo3::PyAny>,
                op: pyo3::basic::CompareOp,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                use pyo3::types::PyAnyMethods;
                let py = other.py();
                if other.is_instance(
                    &<PyContour as pyo3::type_object::PyTypeInfo>::type_object_bound(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyContour>>()?.borrow();
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

            fn __str__(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
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
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyPolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyPolygon>>()?.borrow();
                    let segments =
                        crate::traits::Difference::difference(&self.0, &other.0);
                    Ok(super::unpacking::unpack_maybe_empty_segments::<
                        PyEmpty,
                        PyMultisegment,
                        Segment,
                    >(segments, py))
                } else if other.is_instance(
                    &<PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultipolygon>>()?.borrow();
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
                    &<Self as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, Self>>()?.borrow();
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
                    &<PyMultisegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PyMultisegment>>()?.borrow();
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
                    &<PySegment as pyo3::type_object::PyTypeInfo>::type_object_bound(py),
                )? {
                    let other = other.extract::<pyo3::Bound<'_, PySegment>>()?.borrow();
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
