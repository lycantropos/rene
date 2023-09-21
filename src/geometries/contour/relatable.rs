use std::hash::Hash;
use std::ops::{Div, Neg};

use crate::bounded;
use crate::bounded::Bounded;
use traiter::numbers::Signed;

use crate::geometries::{
    Empty, Multipolygon, Multisegment, Point, Polygon, Segment,
};
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::{contour, linear, mixed, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multipolygonal, MultipolygonalIntoIteratorPolygon,
    Multisegmental, MultisegmentalIndexSegment, Multivertexal,
    MultivertexalIndexVertex, Polygonal, PolygonalContour, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental,
};

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
        contour::relate_to_contour(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Multipolygon<Scalar>> for &Contour<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    mixed::Operation<true, Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<
            IndexSegment = Segment<Scalar>,
            IntoIteratorSegment = &'a Segment<Scalar>,
        >,
    for<'a> &'a Multipolygon<Scalar>:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Multipolygon<Scalar>) -> Relation {
        contour::relate_to_multipolygon(self, other)
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
        contour::relate_to_multisegment(self, other)
    }
}

impl<
        Scalar: Div<Output = Scalar>
            + Hash
            + Neg<Output = Scalar>
            + Ord
            + Square<Output = Scalar>,
    > Relatable<&Polygon<Scalar>> for &Contour<Scalar>
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
    for<'a> &'a Polygon<Scalar>: Polygonal<Contour = &'a Contour<Scalar>, IndexHole =Contour<Scalar>>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Polygon<Scalar>) -> Relation {
        contour::relate_to_polygon(self, other)
    }
}

impl<Scalar> Relatable<&Segment<Scalar>> for &Contour<Scalar>
where
    Point<Scalar>: Clone + PartialOrd,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a> &'a Contour<Scalar>:
        Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Segment<Scalar>) -> Relation {
        contour::relate_to_segment(self, other)
    }
}
