use crate::bounded::{Bounded, Box};
use crate::geometries::Contour;

use super::types::Polygon;

impl<Scalar> Bounded<Scalar> for Polygon<Scalar>
where
    Contour<Scalar>: Bounded<Scalar>,
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
