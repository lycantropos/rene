use std::hash::Hash;

use crate::geometries::{contracts, Contour};

use super::types::Polygon;

impl<Scalar> PartialEq for Polygon<Scalar>
where
    Contour<Scalar>: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.border == other.border
            && contracts::are_unique_hashable_sequences_permutationally_equivalent(
                &self.holes,
                &other.holes,
            )
    }
}
