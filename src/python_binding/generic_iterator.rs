pub(super) enum GenericIterator<I> {
    Forward(I),
    Backward(std::iter::Rev<I>),
}

impl<I: DoubleEndedIterator> Iterator for GenericIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Forward(iterator) => iterator.next(),
            Self::Backward(iterator) => iterator.next(),
        }
    }
}

impl<I: DoubleEndedIterator> GenericIterator<I>
where
    I::Item: PartialEq,
{
    pub(super) fn contains(&mut self, value: I::Item) -> bool {
        self.any(|candidate| candidate == value)
    }
}
