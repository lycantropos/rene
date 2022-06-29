use std::hash::Hash;

use crate::geometries::contracts;

use super::types::Multisegment;

impl<Scalar: Hash + PartialOrd + Eq> PartialEq for Multisegment<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        contracts::are_unique_hashable_sequences_permutationally_equivalent(
            &self.segments,
            &other.segments,
        )
    }
}
