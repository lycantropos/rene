use crate::bounded;
use crate::geometries::Point;
use crate::Elemental;

use super::types::Segment;

impl<Scalar: Clone + Ord> bounded::Bounded<Scalar> for Segment<Scalar>
where
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        self.start.x().max(self.end.x()).clone()
    }

    fn to_max_y(&self) -> Scalar {
        self.start.y().max(self.end.y()).clone()
    }

    fn to_min_x(&self) -> Scalar {
        self.start.x().min(self.end.x()).clone()
    }

    fn to_min_y(&self) -> Scalar {
        self.start.y().min(self.end.y()).clone()
    }
}
