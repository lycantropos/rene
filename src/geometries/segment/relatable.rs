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

impl<'a, Scalar> Relatable<&'a Contour<Scalar>> for &Segment<Scalar>
where
    &'a Contour<Scalar>: Contoural<Segment = Segment<Scalar>, Vertex = Point<Scalar>>,
    Segment<Scalar>: Segmental<Endpoint = Point<Scalar>>,
    Point<Scalar>: Orient + PartialOrd,
{
    fn relate_to(self, other: &'a Contour<Scalar>) -> Relation {
        segment::relate_to_contour(&self.start, &self.end, other)
    }
}
