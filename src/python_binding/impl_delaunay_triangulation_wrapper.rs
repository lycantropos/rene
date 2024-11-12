macro_rules! impl_delaunay_triangulation_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyDelaunayTriangulation {
            #[classmethod]
            #[pyo3(signature = (points, /))]
            fn from_points(
                _: &pyo3::Bound<'_, pyo3::types::PyType>,
                points: &pyo3::Bound<'_, pyo3::types::PySequence>,
            ) -> pyo3::PyResult<Self> {
                Ok(PyDelaunayTriangulation(DelaunayTriangulation::from(
                    super::conversion::extract_from_py_sequence::<
                        Point,
                        PyPoint,
                    >(points)?,
                )))
            }

            #[getter]
            fn border(&self) -> pyo3::PyResult<PyContour> {
                super::packing::try_pack_vertices(
            crate::triangulation::BoundaryEndpoints::get_boundary_endpoints(
                &self.0,
            )
            .into_iter()
            .cloned()
            .collect(),
        )
            }

            #[getter]
            fn triangles(&self) -> Vec<Contour> {
                self.0
                    .iter_triangles_vertices()
                    .map(|(first, second, third)| {
                        Contour::from([
                            first.clone(),
                            second.clone(),
                            third.clone(),
                        ])
                    })
                    .collect()
            }

            fn __bool__(&self) -> bool {
                !self.0.is_empty()
            }
        }
    };
}

pub(super) use impl_delaunay_triangulation_wrapper;
