use crate::bounded;
use crate::geometries::Point;
use crate::Elemental;

use super::types::Segment;

impl<Scalar: Ord> bounded::Bounded<Scalar> for Segment<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        self.start.x().max(self.end.x())
    }

    fn to_max_y(&self) -> Scalar {
        self.start.y().max(self.end.y())
    }

    fn to_min_x(&self) -> Scalar {
        self.start.x().min(self.end.x())
    }

    fn to_min_y(&self) -> Scalar {
        self.start.y().min(self.end.y())
    }
}
