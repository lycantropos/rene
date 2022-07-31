use crate::geometries::Point;
use crate::operations::{relate_segments, Orient};
use crate::relatable::{Relatable, Relation};

use super::types::Segment;

impl<Scalar> Relatable for Segment<Scalar>
where
    Point<Scalar>: Orient + PartialOrd,
{
    fn relate(self, other: Self) -> Relation {
        relate_segments(&self.start, &self.end, &other.start, &other.end)
    }
}
