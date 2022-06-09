use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::operations::relate_segments;
use crate::relatable::{Relatable, Relation};

use super::types::Segment;

impl<Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + PartialOrd + Signed> Relatable
    for Segment<Scalar>
{
    fn relate(self, other: Self) -> Relation {
        relate_segments(&self.start, &self.end, &other.start, &other.end)
    }
}