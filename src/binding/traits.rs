use crate::slice_sequence::SliceSequence;
use crate::traits::Iterable;

pub(super) trait TryFromPyAny
where
    Self: Sized,
{
    fn try_from_py_any(
        value: &pyo3::PyAny,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<Self>;
}

pub(super) trait TryToPyAny {
    fn try_to_py_any(
        self,
        py: pyo3::Python,
    ) -> pyo3::prelude::PyResult<&pyo3::PyAny>;
}

pub(super) trait Count<T> {
    fn count(&self, value: &T) -> usize;
}

impl<'a, T: PartialEq> Count<T> for SliceSequence<'a, T> {
    fn count(&self, value: &T) -> usize {
        self.iter().filter(|&candidate| candidate.eq(value)).count()
    }
}
