use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::operations::{
    to_boxes_ids_with_intersection, CrossMultiply, DotMultiply,
    IntersectCrossingSegments, Orient, Square, SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, Multivertexal, MultivertexalIndexVertex,
    Polygonal, PolygonalIntoIteratorHole, Segmental,
};

use super::{linear, mixed, segment, Event};

pub(crate) fn relate_to_contour<
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
    polygon: &Polygon,
    contour: &Contour,
) -> Relation
where
    mixed::Operation<false, Point>:
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
    relate_to_multisegmental::<
        true,
        Contour,
        Contour,
        Output,
        Point,
        Polygon,
        Scalar,
        Segment,
    >(polygon, contour)
}

pub(crate) fn relate_to_multisegment<
    Border,
    Multisegment,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Polygon,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    polygon: &Polygon,
    multisegment: &Multisegment,
) -> Relation
where
    mixed::Operation<false, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Border>: Elemental,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
    for<'a> &'a Polygon: Bounded<&'a Scalar> + Polygonal<Contour = &'a Border, IndexHole = Border>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    relate_to_multisegmental::<
        false,
        Border,
        Multisegment,
        Output,
        Point,
        Polygon,
        Scalar,
        Segment,
    >(polygon, multisegment)
}

pub(crate) fn relate_to_segment<
    Border,
    Point: Clone + Ord,
    Polygon,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    polygon: &Polygon,
    segment: &Segment,
) -> Relation
where
    for<'a, 'b> &'a MultisegmentalIndexSegment<PolygonalIntoIteratorHole<&'b Polygon>>:
        Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Border>: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<PolygonalIntoIteratorHole<&'b Polygon>>:
        Elemental,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    for<'a> &'a Polygon: Polygonal<Contour = &'a Border, IndexHole = Border>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    segment::relate_to_polygon(segment, polygon).to_complement()
}

fn relate_to_multisegmental<
    const IS_CONTOUR: bool,
    Border,
    Multisegment,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Polygon,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    polygon: &Polygon,
    multisegmental: &Multisegment,
) -> Relation
where
    mixed::Operation<false, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<PolygonalIntoIteratorHole<&'b Polygon>>:
        Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Border>: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<PolygonalIntoIteratorHole<&'b Polygon>>:
        Elemental,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
    for<'a> &'a Polygon: Bounded<&'a Scalar>
        + Polygonal<Contour = &'a Border, IndexHole = Border>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    let polygon_bounding_box = polygon.to_bounding_box();
    let multisegmental_bounding_box = multisegmental.to_bounding_box();
    if multisegmental_bounding_box.disjoint_with(&polygon_bounding_box) {
        return Relation::Disjoint;
    }
    let multisegmental_segments = multisegmental.segments();
    let multisegmental_bounding_boxes = multisegmental_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let intersecting_segments_ids = to_boxes_ids_with_intersection(
        &multisegmental_bounding_boxes,
        &polygon_bounding_box,
    );
    if intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    } else if intersecting_segments_ids.len() == 1 {
        return match segment::relate_to_polygon(
            &multisegmental_segments[intersecting_segments_ids[0]],
            polygon,
        ) {
            Relation::Component => Relation::Touch,
            Relation::Enclosed | Relation::Within => Relation::Cross,
            relation => relation,
        };
    }
    let min_max_x = unsafe {
        intersecting_segments_ids
            .iter()
            .map(|&index| multisegmental_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    }
    .min(polygon_bounding_box.get_max_x());
    let intersecting_segments = intersecting_segments_ids
        .iter()
        .map(|&index| &multisegmental_segments[index])
        .collect::<Vec<_>>();
    let border_segments = polygon.border().segments();
    let holes = polygon.holes();
    let intersecting_holes_segments = holes
        .iter()
        .filter_map(|hole| {
            if hole
                .to_bounding_box()
                .disjoint_with(&multisegmental_bounding_box)
            {
                None
            } else {
                Some(hole.segments())
            }
        })
        .collect::<Vec<_>>();
    mixed::Operation::<false, Point>::from_segments_iterators(
        (
            border_segments.len()
                + intersecting_holes_segments
                    .iter()
                    .map(|hole_segments| hole_segments.len())
                    .sum::<usize>(),
            border_segments.into_iter().cloned().chain(
                intersecting_holes_segments.into_iter().flat_map(
                    |hole_segments| hole_segments.into_iter().cloned(),
                ),
            ),
        ),
        (
            intersecting_segments.len(),
            intersecting_segments.iter().copied().cloned(),
        ),
    )
    .into_relation(
        intersecting_segments.len() == multisegmental_segments.len(),
        min_max_x,
    )
}
