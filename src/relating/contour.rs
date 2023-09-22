use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multipolygonal, MultipolygonalIntoIteratorPolygon,
    Multisegmental, MultisegmentalIndexSegment, Multivertexal,
    MultivertexalIndexVertex, Polygonal, PolygonalContour, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental,
};

use super::event::Event;
use super::{linear, mixed, multisegmental, segment};

pub(crate) fn relate_to_contour<
    Contour,
    Point: Clone + Hash + Ord,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    first: &Contour,
    second: &Contour,
) -> Relation
where
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Contour:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    multisegmental::relate_to_multisegmental::<
        true,
        true,
        Contour,
        Contour,
        Output,
        Point,
        Scalar,
        Segment,
    >(first, second)
}

pub(crate) fn relate_to_multipolygon<
    Contour,
    Multipolygon,
    Point: Clone + Ord,
    Polygon,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    contour: &Contour,
    multipolygon: &Multipolygon,
) -> Relation
where
    mixed::Operation<true, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multipolygon:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    for<'a> &'a Polygon: Bounded<&'a Scalar>
        + Polygonal<Contour = &'a Contour, IntoIteratorHole = &'a Contour>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<MultipolygonalIntoIteratorPolygon<&'b Multipolygon>>,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<MultipolygonalIntoIteratorPolygon<&'b Multipolygon>>,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<&'b PolygonalIndexHole<&'c Polygon>>:
        Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon>>:
        Elemental,
{
    multisegmental::relate_to_multipolygon::<
        false,
        Contour,
        Multipolygon,
        Contour,
        Point,
        Polygon,
        Scalar,
        Segment,
    >(contour, multipolygon)
}

pub(crate) fn relate_to_multisegment<
    Contour,
    Multisegment,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    contour: &Contour,
    multisegment: &Multisegment,
) -> Relation
where
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Contour:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    multisegmental::relate_to_multisegmental::<
        true,
        false,
        Contour,
        Multisegment,
        Output,
        Point,
        Scalar,
        Segment,
    >(contour, multisegment)
}

pub(crate) fn relate_to_polygon<
    Contour,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Polygon,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    contour: &Contour,
    polygon: &Polygon,
) -> Relation
where
    mixed::Operation<true, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour>: Elemental,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Contour: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
    for<'a> &'a Polygon: Bounded<&'a Scalar> + Polygonal<Contour = &'a Contour, IndexHole =Contour>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    multisegmental::relate_to_polygon::<
        true,
        Contour,
        Contour,
        Output,
        Point,
        Polygon,
        Scalar,
        Segment,
    >(contour, polygon)
}

pub(crate) fn relate_to_segment<Contour, Point: Clone + PartialOrd, Segment>(
    contour: &Contour,
    segment: &Segment,
) -> Relation
where
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour>: Segmental,
    for<'a> &'a Contour: Contoural<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Point: Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    segment::relate_to_contour(segment, contour).to_complement()
}
