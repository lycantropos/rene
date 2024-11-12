use std::ops::Index;

use crate::traits::{Iterable, Lengthsome, Sequence};

pub struct SliceSequence<'a, T> {
    slice: &'a [T],
}

impl<'a, T> SliceSequence<'a, T> {
    pub(crate) fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }
}

impl<T> Index<usize> for SliceSequence<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.slice[index]
    }
}

impl<'a, T> IntoIterator for SliceSequence<'a, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.iter()
    }
}

impl<T> Iterable for SliceSequence<'_, T> {
    type Item = T;
    type Output<'a>
        = std::slice::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::Output<'_> {
        self.slice.iter()
    }
}

impl<T> Lengthsome for SliceSequence<'_, T> {
    type Output = usize;

    fn len(&self) -> Self::Output {
        self.slice.len()
    }
}

impl<T: PartialEq> PartialEq for SliceSequence<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.slice, other.slice) || self.slice.eq(other.slice)
    }
}

impl<T: Eq> Eq for SliceSequence<'_, T> {}

impl<'a, T> Sequence for SliceSequence<'a, T> {
    type IndexItem = T;
    type IntoIteratorItem = &'a T;
}
