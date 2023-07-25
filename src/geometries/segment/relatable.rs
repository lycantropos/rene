use std::hash::Hash;
use std::ops::Div;

use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{CrossMultiply, Orient};
use crate::relatable::{Relatable, Relation};
use crate::relating::segment;
use crate::traits::{Contoural2, Elemental, Multisegmental, Segmental};

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
        segment::relate_to_segment(
            &self.start,
            &self.end,
            &other.start,
            &other.end,
        )
    }
}

impl<'a, Scalar> Relatable<&'a Contour<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + PartialOrd,
    for<'b> &'b Contour<Scalar>: Contoural2<IndexSegment = Segment<Scalar>>,
    for<'b> &'b Point<Scalar>: Orient,
    for<'b> &'b Segment<Scalar>: Segmental<Endpoint = &'b Point<Scalar>>,
{
    fn relate_to(self, other: &'a Contour<Scalar>) -> Relation {
        segment::relate_to_contour(&self.start, &self.end, other)
    }
}

impl<'a, Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd>
    Relatable<&'a Multisegment<Scalar>> for &'a Segment<Scalar>
where
    Self: Segmental<Endpoint = &'a Point<Scalar>>,
    &'a Multisegment<Scalar>: Multisegmental<Segment = &'a Segment<Scalar>>,
    Point<Scalar>: Eq + Hash + Ord,
    for<'b> &'b Point<Scalar>: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    fn relate_to(self, other: &'a Multisegment<Scalar>) -> Relation {
        segment::relate_to_multisegment(&self.start, &self.end, other)
    }
}
