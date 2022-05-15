use rithm::traits::{AdditiveGroup, MultiplicativeMonoid};

use crate::geometries::Point;

pub(super) fn cross_multiply<Scalar: AdditiveGroup + MultiplicativeMonoid>(
    first_start: Point<Scalar>,
    first_end: Point<Scalar>,
    second_start: Point<Scalar>,
    second_end: Point<Scalar>,
) -> Scalar {
    (first_end.0 - first_start.0) * (second_end.1 - second_start.1)
        - (first_end.1 - first_start.1) * (second_end.0 - second_start.0)
}
