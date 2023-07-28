use std::ops::Index;

use crate::traits::{Iterable, Lengthsome, Sequence};

pub struct SliceSequence<'a, T> {
    slice: &'a [T],
}

impl<'a, T> SliceSequence<'a, T> {
    pub(crate) fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }

    pub(crate) fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.slice.contains(value)
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

impl<'a, T: PartialEq> PartialEq for SliceSequence<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.slice, other.slice) || self.slice.eq(other.slice)
    }
}

impl<'a, T: Eq> Eq for SliceSequence<'a, T> {}

impl<'a, T> Sequence for SliceSequence<'a, T> {
    type IndexItem = T;
    type IntoIteratorItem = &'a T;
}
