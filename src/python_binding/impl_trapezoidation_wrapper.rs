macro_rules! impl_trapezoidation_wrapper {
    () => {
        #[pyo3::pymethods]
        impl PyTrapezoidation {
            #[classmethod]
            #[pyo3(signature = (multisegment, seed, /))]
            fn from_multisegment(
                _: &pyo3::Bound<'_, pyo3::types::PyType>,
                multisegment: &PyMultisegment,
                seed: usize,
            ) -> Self {
                PyTrapezoidation(Trapezoidation::from_multisegment(
                    &multisegment.0,
                    |values| crate::operations::permute(values, seed),
                ))
            }

            #[classmethod]
            #[pyo3(signature = (polygon, seed, /))]
            fn from_polygon(
                _: &pyo3::Bound<'_, pyo3::types::PyType>,
                polygon: &PyPolygon,
                seed: usize,
            ) -> Self {
                PyTrapezoidation(Trapezoidation::from_polygon(
                    &polygon.0,
                    |values| {
                        crate::operations::permute(values, seed);
                    },
                ))
            }

            #[getter]
            fn height(&self) -> usize {
                self.0.height()
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

            fn __contains__(&self, point: &pyo3::Bound<'_, PyPoint>) -> bool {
                crate::locatable::Locatable::locate(&self.0, &point.borrow().0)
                    != crate::locatable::Location::Exterior
            }
        }
    };
}

pub(super) use impl_trapezoidation_wrapper;
