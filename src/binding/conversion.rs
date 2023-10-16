pub(super) fn extract_from_py_sequence<
    'a,
    Wrapped: From<Wrapper>,
    Wrapper: pyo3::FromPyObject<'a>,
>(
    sequence: &'a pyo3::types::PySequence,
) -> pyo3::prelude::PyResult<Vec<Wrapped>> {
    let mut result = Vec::<Wrapped>::with_capacity(sequence.len()?);
    for element in sequence.iter()? {
        result.push(element?.extract::<Wrapper>()?.into());
    }
    Ok(result)
}
