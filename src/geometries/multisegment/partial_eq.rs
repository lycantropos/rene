use std::hash::Hash;

use crate::geometries::{contracts, Segment};

use super::types::Multisegment;

impl<Scalar> PartialEq for Multisegment<Scalar>
where
    Segment<Scalar>: Hash + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        contracts::are_unique_hashable_sequences_permutationally_equivalent(
            &self.segments,
            &other.segments,
        )
    }
}
