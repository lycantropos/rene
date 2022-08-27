use crate::bounded::{Bounded, Box};
use crate::geometries::Point;
use crate::Elemental;

use super::types::Polygon;

impl<Scalar: Ord> Bounded<Scalar> for Polygon<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        self.border.to_max_x()
    }

    fn to_max_y(&self) -> Scalar {
        self.border.to_max_y()
    }

    fn to_min_x(&self) -> Scalar {
        self.border.to_min_x()
    }

    fn to_min_y(&self) -> Scalar {
        self.border.to_min_y()
    }

    fn to_bounding_box(&self) -> Box<Scalar> {
        self.border.to_bounding_box()
    }
}
