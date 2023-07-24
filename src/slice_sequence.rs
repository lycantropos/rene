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

impl<'a, T> Index<usize> for SliceSequence<'a, T> {
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

impl<'a, T> Iterable for SliceSequence<'a, T> {
    type Item = T;
    type Output<'b> = std::slice::Iter<'b, T>
    where
        Self: 'b,
        T: 'b;

    fn iter(&self) -> Self::Output<'_> {
        self.slice.iter()
    }
}

impl<'a, T> Lengthsome for SliceSequence<'a, T> {
    type Output = usize;

    fn len(&self) -> Self::Output {
        self.slice.len()
    }
}

impl<'a, T> Sequence for SliceSequence<'a, T> {
    type IndexItem = T;
    type IntoIteratorItem = &'a T;
}
