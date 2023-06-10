use crate::geometries::Point;
use crate::operations::Orient;
use crate::relatable::{Relatable, Relation};
use crate::relating::segment;

use super::types::Segment;

impl<Scalar> Relatable for &Segment<Scalar>
where
    Point<Scalar>: Orient + PartialOrd,
{
    fn equals_to(self, other: Self) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end)
            || self.start.eq(&other.end) && self.end.eq(&other.start)
    }

    fn relate_to(self, other: Self) -> Relation {
        segment::relate_segment(&self.start, &self.end, &other.start, &other.end)
    }
}
