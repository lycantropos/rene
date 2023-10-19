macro_rules! impl_polygon_wrapper {
    () => {
        #[pyo3::prelude::pymethods]
        impl PyPolygon {
            #[new]
            #[pyo3(signature = (border, holes, /))]
            fn new(
                border: &PyContour,
                holes: &pyo3::types::PySequence,
            ) -> pyo3::PyResult<Self> {
                Ok(PyPolygon(Polygon::new(
                    border.0.clone(),
                    super::conversion::extract_from_py_sequence::<
                        Contour,
                        PyContour,
                    >(holes)?,
                )))
            }

            #[getter]
            fn border(&self) -> Contour {
                (&self.0).border().clone()
            }

            #[getter]
            fn bounding_box(&self) -> Box {
                crate::bounded::Bounded::to_bounding_box(&self.0).cloned()
            }

            #[getter]
            fn holes(slf: pyo3::PyRef<Self>) -> PyPolygonHoles {
                let holes_count =
                    crate::traits::Lengthsome::len(&(&slf.0).holes());
                PyPolygonHoles {
                    polygon: PyPolygonReference::from_py_ref(slf),
                    start: 0isize,
                    stop: holes_count as isize,
                    step: 1isize,
                }
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
                if other.is_instance_of::<Self>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<Self>>()?.0,
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
                } else if other.is_instance_of::<PyContour>() {
                    TryToPyAny::try_to_py_any(
                        crate::relatable::Relatable::relate_to(
                            &self.0,
                            &other.extract::<pyo3::PyRef<PyContour>>()?.0,
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
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Intersection::intersection(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
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
                pyo3::types::PyTuple::new(
                    py,
                    &[
                        pyo3::IntoPy::into_py(self.border(), py),
                        pyo3::IntoPy::into_py(
                            pyo3::types::PyFrozenSet::new(
                                py,
                                (&self.0).holes(),
                            )?,
                            py,
                        ),
                    ],
                )
                .hash()
            }

            fn __or__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Union::union(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons = crate::traits::Union::union(&self.0, &other.0);
            Ok(super::unpacking::unpack_non_empty_polygons::<
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
            }

            fn __repr__(&self, py: pyo3::Python) -> pyo3::PyResult<String> {
                Ok(format!(
                    "{}({}, {})",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    PyContour(self.border()).__repr__(py)?,
                    pyo3::IntoPy::into_py(
                        crate::traits::Iterable::iter(&(&self.0).holes())
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
                    <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(
                        py,
                    ),
                )? {
                    let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
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
                    "{}({}, [{}])",
                    <Self as pyo3::type_object::PyTypeInfo>::NAME,
                    PyContour(self.border()).__str__(py)?,
                    crate::traits::Iterable::iter(&(&self.0).holes())
                        .cloned()
                        .map(|hole| PyContour(hole).__str__(py))
                        .collect::<pyo3::PyResult<Vec<String>>>()?
                        .join(", ")
                ))
            }

            fn __sub__(
                &self,
                other: &pyo3::PyAny,
            ) -> pyo3::PyResult<pyo3::PyObject> {
                let py = other.py();
                if other.is_instance(
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::Difference::difference(&self.0, &other.0),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::Difference::difference(&self.0, &other.0);
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
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
            <PyEmpty as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyEmpty>>()?;
            Ok(pyo3::IntoPy::into_py(
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                ),
                py,
            ))
        } else if other.is_instance(
            <PyMultipolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyMultipolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else if other.is_instance(
            <PyPolygon as pyo3::type_object::PyTypeInfo>::type_object(py),
        )? {
            let other = other.extract::<pyo3::PyRef<PyPolygon>>()?;
            let polygons =
                crate::traits::SymmetricDifference::symmetric_difference(
                    &self.0, &other.0,
                );
            Ok(super::unpacking::unpack_maybe_empty_polygons::<
                PyEmpty,
                PyMultipolygon,
                Polygon,
            >(polygons, py))
        } else {
            Ok(py.NotImplemented())
        }
            }
        }
    };
}

pub(super) use impl_polygon_wrapper;
