pub(super) fn extract_from_py_sequence<
    'a,
    Wrapped: From<Wrapper>,
    Wrapper: pyo3::FromPyObject<'a>,
>(
    sequence: &'a pyo3::Bound<'_, pyo3::types::PySequence>,
) -> pyo3::PyResult<Vec<Wrapped>> {
    use pyo3::types::PyAnyMethods;
    let mut result = Vec::<Wrapped>::with_capacity(sequence.len()?);
    for element in sequence.try_iter()? {
        result.push(element?.extract::<Wrapper>()?.into());
    }
    Ok(result)
}
