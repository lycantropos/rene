use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::{Contour, Empty, Point, Polygon, Segment};
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::{linear, mixed, multisegment, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multisegmental, MultisegmentalIndexSegment,
    Multivertexal, MultivertexalIndexVertex, Polygonal,
    PolygonalIntoIteratorHole, Segmental,
};

use super::types::Multisegment;

impl<Scalar> Relatable<&Empty> for &Multisegment<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar: PartialOrd> Relatable for &Multisegment<Scalar>
where
    Point<Scalar>: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar>
        + Hash
        + Neg<Output = Scalar>
        + Ord
        + Square<Output = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Multisegment<Scalar>:
        Multisegmental<IndexSegment = Segment<Scalar>>,
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

impl<Scalar: PartialOrd> Relatable<&Contour<Scalar>> for &Multisegment<Scalar>
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
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        multisegment::relate_to_contour(self, other)
    }
}

impl<
        Scalar: Div<Output = Scalar>
            + Hash
            + Neg<Output = Scalar>
            + Ord
            + Square<Output = Scalar>,
    > Relatable<&Polygon<Scalar>> for &Multisegment<Scalar>
where
    Point<Scalar>: Clone + Hash + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    mixed::Operation<true, Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> linear::Operation<Point<Scalar>>: From<(&'a [&'b Segment<Scalar>], &'a [&'b Segment<Scalar>])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment<Scalar>, IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Multisegment<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Scalar>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient
        + SquaredMetric<Output = Scalar>,
    for<'a> &'a Polygon<Scalar>: Polygonal<Contour = &'a Contour<Scalar>, IndexHole = Contour<Scalar>>,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Polygon<Scalar>) -> Relation {
        multisegment::relate_to_polygon(self, other)
    }
}

impl<'a, Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd>
    Relatable<&'a Segment<Scalar>> for &'a Multisegment<Scalar>
where
    Point<Scalar>: Hash + Ord,
    &'a Multisegment<Scalar>:
        Multisegmental<IntoIteratorSegment = &'a Segment<Scalar>>,
    &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'b> &'b MultisegmentalIndexSegment<Self>: Segmental,
    for<'b> &'b Point<Scalar>: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    fn relate_to(self, other: &'a Segment<Scalar>) -> Relation {
        multisegment::relate_to_segment(self, other)
    }
}
