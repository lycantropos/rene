use std::hash::Hash;

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::geometries::contracts;

use super::types::Polygon;

impl<Scalar: AdditiveGroup + Clone + Eq + Hash + MultiplicativeMonoid + Ord + Signed> PartialEq
    for Polygon<Scalar>
{
    fn eq(&self, other: &Self) -> bool {
        self.border == other.border
            && contracts::are_unique_hashable_sequences_permutationally_equivalent(
                &self.holes,
                &other.holes,
            )
    }
}
