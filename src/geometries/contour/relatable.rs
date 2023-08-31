use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::geometries::{Empty, Multisegment, Point, Segment};
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::{linear, multisegment, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{Multisegmental, Segmental};

use super::types::Contour;

impl<Scalar> Relatable<&Empty> for &Contour<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar: PartialOrd> Relatable for &Contour<Scalar>
where
    Point<Scalar>: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar>
        + Hash
        + Neg<Output = Scalar>
        + Ord
        + Square<Output = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Contour<Scalar>:
        Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> linear::Operation<Point<Scalar>>: From<(&'a [&'b Segment<Scalar>], &'a [&'b Segment<Scalar>])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
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

impl<Scalar: PartialOrd> Relatable<&Multisegment<Scalar>> for &Contour<Scalar>
where
    Point<Scalar>: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar>
        + Hash
        + Neg<Output = Scalar>
        + Ord
        + Square<Output = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Contour<Scalar>:
        Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Multisegment<Scalar>:
        Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient
        + SquaredMetric<Output = Scalar>,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> linear::Operation<Point<Scalar>>: From<(&'a [&'b Segment<Scalar>], &'a [&'b Segment<Scalar>])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
{
    fn relate_to(self, other: &Multisegment<Scalar>) -> Relation {
        multisegment::relate_to_multisegment(self, other)
    }
}
