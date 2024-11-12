pub(super) trait TryFromPyAny
where
    Self: Sized,
{
    fn try_from_py_any(
        value: &pyo3::Bound<'_, pyo3::PyAny>,
        py: pyo3::Python<'_>,
    ) -> pyo3::PyResult<Self>;
}

pub(super) trait TryToPyAny {
    fn try_to_py_any(
        self,
        py: pyo3::Python<'_>,
    ) -> pyo3::PyResult<pyo3::Bound<'_, pyo3::PyAny>>;
}
