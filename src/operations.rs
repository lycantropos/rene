use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Sign, Signed};

use crate::oriented::Orientation;
use crate::traits;

pub(crate) fn cross_multiply<
    Scalar: AdditiveGroup + MultiplicativeMonoid,
    Point: traits::Point<Scalar>,
>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Scalar {
    (first_end.x() - first_start.x()) * (second_end.y() - second_start.y())
        - (first_end.y() - first_start.y()) * (second_end.x() - second_start.x())
}

pub(crate) fn orient<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: traits::Point<Scalar>,
>(
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
) -> Orientation {
    match cross_multiply::<Scalar, Point>(vertex, first_ray_point, vertex, second_ray_point).sign()
    {
        Sign::Negative => Orientation::Clockwise,
        Sign::Positive => Orientation::Counterclockwise,
        Sign::Zero => Orientation::Collinear,
    }
}
