macro_rules! impl_constrained_delaunay_triangulation_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyConstrainedDelaunayTriangulation {
            #[classmethod]
            #[pyo3(signature = (polygon, /))]
            fn from_polygon(
                _: &pyo3::Bound<'_, pyo3::types::PyType>,
                polygon: &PyPolygon,
            ) -> Self {
                PyConstrainedDelaunayTriangulation(
                    ConstrainedDelaunayTriangulation::from(&polygon.0),
                )
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
                    .to_triangles_vertices()
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

pub(super) use impl_constrained_delaunay_triangulation_wrapper;
