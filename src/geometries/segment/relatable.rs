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
    Point<Scalar>: PartialOrd,
    for<'a> &'a Point<Scalar>: Orient,
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
    Point<Scalar>: Clone + PartialOrd,
    for<'b> &'b Contour<Scalar>:
        Contoural<Segment = &'b Segment<Scalar>, Vertex = &'b Point<Scalar>>,
    for<'b> &'b Point<Scalar>: Orient,
    for<'b> &'b Segment<Scalar>: Segmental<Endpoint = &'b Point<Scalar>>,
{
    fn relate_to(self, other: &'a Contour<Scalar>) -> Relation {
        segment::relate_to_contour(&self.start, &self.end, other)
    }
}
