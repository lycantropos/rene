use std::hash::Hash;

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use super::types::Polygon;

impl<Scalar: AdditiveGroup + Clone + Eq + Hash + MultiplicativeMonoid + Ord + Signed> Eq
    for Polygon<Scalar>
{
}
