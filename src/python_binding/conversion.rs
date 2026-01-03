pub(super) fn extract_from_py_sequence<
    'py,
    Wrapped: From<Wrapper>,
    Wrapper: pyo3::conversion::FromPyObjectOwned<'py>,
>(
    sequence: &'_ pyo3::Bound<'py, pyo3::types::PySequence>,
) -> pyo3::PyResult<Vec<Wrapped>> {
    use pyo3::types::PyAnyMethods;
    let mut result = Vec::<Wrapped>::with_capacity(sequence.len()?);
    for element in sequence.try_iter()? {
        result.push(element?.extract::<Wrapper>().map_err(Into::into)?.into());
    }
    Ok(result)
}
