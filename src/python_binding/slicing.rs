use pyo3::exceptions::{PyIndexError, PyOverflowError};
use pyo3::PyErr;

pub(super) fn py_long_to_valid_index(
    value: pyo3::Bound<'_, pyo3::types::PyLong>,
    elements_count: usize,
) -> pyo3::PyResult<usize> {
    use pyo3::types::PyAnyMethods;
    if let Ok(index) = value.extract::<isize>() {
        let elements_count = elements_count as isize;
        if !(-elements_count <= index && index < elements_count) {
            Err(PyIndexError::new_err(format!(
                "Index {} is out of range({}, {}).",
                index, -elements_count, elements_count
            )))
        } else {
            Ok((if index < 0 {
                index + elements_count
            } else {
                index
            }) as usize)
        }
    } else {
        Err(PyIndexError::new_err(format!(
            "Index {} is out of index integer range({}, {}).",
            value.repr()?,
            isize::MIN,
            (isize::MAX as usize) + 1
        )))
    }
}

pub(super) fn normalize_index_start(
    start: Option<&pyo3::Bound<'_, pyo3::types::PyLong>>,
    elements_count: usize,
) -> usize {
    use pyo3::types::PyAnyMethods;
    start
        .map(|value| {
            value
                .extract::<isize>()
                .map(|value| {
                    (if value < 0 {
                        (value + (elements_count as isize)).max(0)
                    } else {
                        value
                    }) as usize
                })
                .unwrap_or(elements_count)
        })
        .unwrap_or(0usize)
}

pub(super) fn normalize_index_stop(
    start: Option<&pyo3::Bound<'_, pyo3::types::PyLong>>,
    elements_count: usize,
) -> usize {
    use pyo3::types::PyAnyMethods;
    start
        .map(|value| {
            value
                .extract::<isize>()
                .map(|value| {
                    (if value < 0 {
                        (value + (elements_count as isize)).max(0)
                    } else {
                        value
                    }) as usize
                })
                .unwrap_or(0)
        })
        .unwrap_or(elements_count)
}

pub(super) fn to_next_slice_indices(
    start: isize,
    step: isize,
    length: usize,
    slice: pyo3::Bound<'_, pyo3::types::PySlice>,
) -> Result<(isize, isize, isize), PyErr> {
    use pyo3::types::PySliceMethods;
    let indices = slice.indices(length as isize)?;
    let result_step = try_multiply_isizes(step, indices.step)?;
    let result_start =
        try_sum_isizes(start, try_multiply_isizes(step, indices.start)?)?;
    let result_stop =
        try_sum_isizes(start, try_multiply_isizes(step, indices.stop)?)?;
    Ok((result_start, result_stop, result_step))
}

fn try_multiply_isizes(first: isize, second: isize) -> pyo3::PyResult<isize> {
    if let (result, false) = first.overflowing_mul(second) {
        Ok(result)
    } else {
        Err(PyOverflowError::new_err(format!(
            "Multiplication of {} & {} is out of range({}, {}).",
            first,
            second,
            isize::MIN,
            (isize::MAX as usize) + 1,
        )))
    }
}

fn try_sum_isizes(first: isize, second: isize) -> pyo3::PyResult<isize> {
    if let (result, false) = first.overflowing_add(second) {
        Ok(result)
    } else {
        Err(PyOverflowError::new_err(format!(
            "Addition of {} & {} is out of range({}, {}).",
            first,
            second,
            isize::MIN,
            (isize::MAX as usize) + 1,
        )))
    }
}
