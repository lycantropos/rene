use crate::geometries::contracts;

use super::types::Contour;

impl<Scalar: PartialEq> PartialEq for Contour<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        contracts::are_non_empty_unique_sequences_rotationally_equivalent(&self.0, &other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        !contracts::are_non_empty_unique_sequences_rotationally_equivalent(&self.0, &other.0)
    }
}
