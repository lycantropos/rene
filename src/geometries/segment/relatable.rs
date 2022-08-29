use crate::geometries::Point;
use crate::operations::{segment_in_segment, Orient};
use crate::relatable::{Relatable, Relation};

use super::types::Segment;

impl<Scalar> Relatable for &Segment<Scalar>
where
    Point<Scalar>: Orient + PartialOrd,
{
    fn relate_to(self, other: Self) -> Relation {
        segment_in_segment(&self.start, &self.end, &other.start, &other.end)
    }
}
