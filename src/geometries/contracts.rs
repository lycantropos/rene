use std::collections::hash_map::RandomState;
use std::collections::hash_set::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub(super) fn are_non_empty_unique_sequences_rotationally_equivalent<T: PartialEq>(
    left: &[T],
    right: &[T],
) -> bool {
    debug_assert!(!left.is_empty() && !right.is_empty());
    if left.len() != right.len() {
        false
    } else {
        let left_first_element = &left[0];
        right
            .iter()
            .position(|value| value == left_first_element)
            .map(|index| {
                (left[1..left.len() - index] == right[index + 1..]
                    && left[left.len() - index..] == right[..index])
                    || (left[left.len() - index..]
                        .iter()
                        .rev()
                        .eq(right[..index].iter())
                        && left[1..left.len() - index]
                            .iter()
                            .rev()
                            .eq(right[index + 1..].iter()))
            })
            .unwrap_or(false)
    }
}

pub(super) fn are_unique_hashable_sequences_permutationally_equivalent<T: Eq + Hash>(
    left: &[T],
    right: &[T],
) -> bool {
    if left.len() != right.len() {
        false
    } else {
        let left_set = HashSet::<_, RandomState>::from_iter(left);
        right.iter().all(|value| left_set.contains(value))
    }
}