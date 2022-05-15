use std::hash::Hash;

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::geometries::contracts;

use super::types::Polygon;

impl<Scalar: AdditiveGroup + Clone + Eq + Hash + MultiplicativeMonoid + Ord + Signed> PartialEq
    for Polygon<Scalar>
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && contracts::are_unique_hashable_sequences_permutationally_equivalent(
                &self.1, &other.1,
            )
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
            || !contracts::are_unique_hashable_sequences_permutationally_equivalent(
                &self.1, &other.1,
            )
    }
}
