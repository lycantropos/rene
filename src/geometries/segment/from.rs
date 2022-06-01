use crate::geometries::Point;

use super::types::Segment;

impl<Scalar> From<(Point<Scalar>, Point<Scalar>)> for Segment<Scalar> {
    fn from((start, end): (Point<Scalar>, Point<Scalar>)) -> Self {
        Self::new(start, end)
    }
}
