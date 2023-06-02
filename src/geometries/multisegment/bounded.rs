use crate::bounded::Bounded;
use crate::geometries::Segment;

use super::types::Multisegment;

impl<Scalar: Ord> Bounded<Scalar> for Multisegment<Scalar>
where
    Segment<Scalar>: Bounded<Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(Bounded::to_max_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(&self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(Bounded::to_max_y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(&self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(Bounded::to_min_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(&self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(Bounded::to_min_y)
                .max()
                .unwrap_unchecked()
        }
    }
}
