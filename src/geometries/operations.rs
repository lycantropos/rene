use rithm::traits::{AdditiveGroup, MultiplicativeMonoid};

use crate::geometries::Point;

pub(super) fn cross_multiply<Scalar: AdditiveGroup + MultiplicativeMonoid>(
    first_start: Point<Scalar>,
    first_end: Point<Scalar>,
    second_start: Point<Scalar>,
    second_end: Point<Scalar>,
) -> Scalar {
    (first_end.x - first_start.x) * (second_end.y - second_start.y)
        - (first_end.y - first_start.y) * (second_end.x - second_start.x)
}
