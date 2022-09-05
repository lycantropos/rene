use crate::bounded::Bounded;
use crate::geometries::Polygon;

use super::types::Multipolygon;

impl<Scalar: Ord> Bounded<Scalar> for Multipolygon<Scalar>
where
    Polygon<Scalar>: Bounded<Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_max_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(&self) -> Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_max_y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(&self) -> Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_min_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(&self) -> Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_min_y)
                .max()
                .unwrap_unchecked()
        }
    }
}
