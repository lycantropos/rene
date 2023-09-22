use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::{Contour, Empty, Multisegment, Point, Segment};
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::oriented::Oriented;
use crate::relatable::{Relatable, Relation};
use crate::relating::{linear, mixed, polygon, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multisegmental, Multivertexal,
    MultivertexalIndexVertex, Polygonal, PolygonalIntoIteratorHole, Segmental,
};

use super::types::Polygon;

impl<
        Scalar: Div<Output = Scalar>
            + Hash
            + Neg<Output = Scalar>
            + Ord
            + Square<Output = Scalar>,
    > Relatable<&Contour<Scalar>> for &Polygon<Scalar>
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
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Scalar>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient
        + SquaredMetric<Output = Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar> + Polygonal<Contour = &'a Contour<Scalar>, IndexHole =Contour<Scalar>>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        polygon::relate_to_contour(self, other)
    }
}

impl<Scalar> Relatable<&Empty> for &Polygon<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<
        Scalar: Div<Output = Scalar>
            + Hash
            + Neg<Output = Scalar>
            + Ord
            + Square<Output = Scalar>,
    > Relatable<&Multisegment<Scalar>> for &Polygon<Scalar>
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
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar> + Polygonal<Contour = &'a Contour<Scalar>, IndexHole = Contour<Scalar>>,
    for<'a> &'a Scalar: Signed,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Multisegment<Scalar>) -> Relation {
        polygon::relate_to_multisegment(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Segment<Scalar>> for &Polygon<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<
            IndexSegment = Segment<Scalar>,
            IntoIteratorSegment = &'a Segment<Scalar>,
        > + Oriented,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>:
        Polygonal<Contour = &'a Contour<Scalar>, IndexHole = Contour<Scalar>>,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Segment<Scalar>) -> Relation {
        polygon::relate_to_segment(self, other)
    }
}
