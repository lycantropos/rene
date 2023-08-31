pub(super) fn all_equal<I: Iterator>(mut iterator: I) -> bool
where
    <I as Iterator>::Item: PartialEq,
{
    match iterator.next() {
        None => true,
        Some(first_element) => {
            iterator.all(|element| element == first_element)
        }
    }
}
