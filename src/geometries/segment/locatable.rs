use crate::geometries::Point;
use crate::locatable::{Locatable, Location};
use crate::operations::{is_point_in_segment, Orient};
use crate::traits::Elemental;

use super::types::Segment;

impl<Scalar: PartialOrd> Locatable<&Point<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar> + PartialEq,
    for<'a> &'a Point<Scalar>: Orient,
{
    fn locate(self, point: &Point<Scalar>) -> Location {
        if is_point_in_segment(point, &self.start, &self.end) {
            Location::Boundary
        } else {
            Location::Exterior
        }
    }
}
