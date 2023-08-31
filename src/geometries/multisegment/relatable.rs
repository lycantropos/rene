use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::geometries::{Empty, Point, Segment};
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::{linear, multisegment, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{Multisegmental, Segmental};

use super::types::Multisegment;

impl<Scalar> Relatable<&Empty> for &Multisegment<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar: PartialOrd> Relatable for &Multisegment<Scalar>
where
    Scalar: Div<Output = Scalar>
        + Hash
        + Neg<Output = Scalar>
        + Ord
        + Square<Output = Scalar>,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Multisegment<Scalar>:
        Multisegmental<IndexSegment = Segment<Scalar>>,
    Segment<Scalar>: Clone,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> linear::Operation<Point<Scalar>>: From<(&'a [&'b Segment<Scalar>], &'a [&'b Segment<Scalar>])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    Point<Scalar>: Clone + Hash + Ord,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient
        + SquaredMetric<Output = Scalar>,
{
    fn relate_to(self, other: Self) -> Relation {
        multisegment::relate_to_multisegment(self, other)
    }
}
