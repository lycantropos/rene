use crate::geometries::{Contour, Empty, Point};
use crate::operations::Orient;
use crate::relatable::{Relatable, Relation};
use crate::relating::segment;
use crate::traits::{Contoural, Segmental};

use super::types::Segment;

impl<Scalar> Relatable<&Empty> for &Segment<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable for &Segment<Scalar>
where
    Point<Scalar>: Orient + PartialOrd,
{
    fn equals_to(self, other: Self) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end)
            || self.start.eq(&other.end) && self.end.eq(&other.start)
    }

    fn relate_to(self, other: Self) -> Relation {
        segment::relate_to_segment(&self.start, &self.end, &other.start, &other.end)
    }
}

impl<Scalar> Relatable<&Contour<Scalar>> for &Segment<Scalar>
where
    Contour<Scalar>: Contoural<Segment = Segment<Scalar>, Vertex = Point<Scalar>>,
    Point<Scalar>: Orient + PartialOrd,
    Segment<Scalar>: Segmental<Endpoint = Point<Scalar>>,
{
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        segment::relate_to_contour(&self.start, &self.end, other)
    }
}
