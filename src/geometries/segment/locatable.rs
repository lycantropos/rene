use crate::geometries::Point;
use crate::locatable::{Locatable, Location};
use crate::operations::Orient;
use crate::oriented::Orientation;
use crate::traits::Elemental;

use super::types::Segment;

impl<Scalar: PartialOrd> Locatable<&Point<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar> + Orient + PartialEq,
{
    fn locate(self, other: &Point<Scalar>) -> Location {
        if self.start.eq(other)
            || self.end.eq(other)
            || ({
                let start_x = self.start.x();
                let end_x = self.end.x();
                let point_x = other.x();
                if start_x <= end_x {
                    start_x <= point_x && point_x <= end_x
                } else {
                    end_x < point_x && point_x < start_x
                }
            } && {
                let start_y = self.start.y();
                let end_y = self.end.y();
                let point_y = other.y();
                if start_y <= end_y {
                    start_y <= point_y && point_y <= end_y
                } else {
                    end_y < point_y && point_y < start_y
                }
            } && self.start.orient(&self.end, &other) == Orientation::Collinear)
        {
            Location::Boundary
        } else {
            Location::Exterior
        }
    }
}
