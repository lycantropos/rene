use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Point;
use crate::operations::segmental_to_bounds;
use crate::traits::{Elemental, Segmental};

use super::types::Segment;

impl<'a, Scalar: Ord> Bounded<&'a Scalar> for &'a Segment<Scalar>
where
    &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
    &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn to_bounding_box(self) -> bounded::Box<&'a Scalar> {
        let (min_x, max_x, min_y, max_y) = segmental_to_bounds(self);
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> &'a Scalar {
        self.start.x().max(self.end.x())
    }

    fn to_max_y(self) -> &'a Scalar {
        self.start.y().max(self.end.y())
    }

    fn to_min_x(self) -> &'a Scalar {
        self.start.x().min(self.end.x())
    }

    fn to_min_y(self) -> &'a Scalar {
        self.start.y().min(self.end.y())
    }
}

impl<Scalar: Ord> Bounded<Scalar> for Segment<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Segmental<Endpoint = Point<Scalar>>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (min_x, max_x, min_y, max_y) = segmental_to_bounds(self);
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> Scalar {
        self.start.x().max(self.end.x())
    }

    fn to_max_y(self) -> Scalar {
        self.start.y().max(self.end.y())
    }

    fn to_min_x(self) -> Scalar {
        self.start.x().min(self.end.x())
    }

    fn to_min_y(self) -> Scalar {
        self.start.y().min(self.end.y())
    }
}
