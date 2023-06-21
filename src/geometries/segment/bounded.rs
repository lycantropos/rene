use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Point;
use crate::operations::to_sorted_pair;
use crate::traits::Elemental;

use super::types::Segment;

impl<'a, Scalar: Clone + Ord> Bounded<Scalar> for &'a Segment<Scalar>
where
    &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        bounded::Box::new(
            self.to_min_x(),
            self.to_max_x(),
            self.to_min_y(),
            self.to_max_y(),
        )
    }

    fn to_max_x(self) -> Scalar {
        self.start.x().max(self.end.x()).clone()
    }

    fn to_max_y(self) -> Scalar {
        self.start.y().max(self.end.y()).clone()
    }

    fn to_min_x(self) -> Scalar {
        self.start.x().min(self.end.x()).clone()
    }

    fn to_min_y(self) -> Scalar {
        self.start.y().min(self.end.y()).clone()
    }
}

impl<'a, Scalar: Ord> Bounded<Scalar> for Segment<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (start_x, start_y) = self.start.coordinates();
        let (end_x, end_y) = self.end.coordinates();
        let (max_x, min_x) = to_sorted_pair((start_x, end_x));
        let (max_y, min_y) = to_sorted_pair((start_y, end_y));
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
