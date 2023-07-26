use std::hash::Hash;
use std::ops::Div;

use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{CrossMultiply, Orient};
use crate::relatable::{Relatable, Relation};
use crate::relating::segment;
use crate::traits::{
    Contoural, Elemental, Multisegmental, MultisegmentalIndexSegment,
    Segmental,
};

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

impl<Scalar> Relatable<&Contour<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + PartialOrd,
    for<'a> &'a Contour<Scalar>: Contoural<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        segment::relate_to_contour(&self.start, &self.end, other)
    }
}

impl<Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd>
    Relatable<&Multisegment<Scalar>> for &Segment<Scalar>
where
    for<'a> &'a Multisegment<Scalar>:
        Multisegmental<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Multisegment<Scalar>>:
        Segmental,
    Point<Scalar>: Eq + Hash + Ord,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'a Scalar>
        + Orient,
{
    fn relate_to(self, other: &Multisegment<Scalar>) -> Relation {
        segment::relate_to_multisegment(&self.start, &self.end, other)
    }
}
