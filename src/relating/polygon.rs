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
    Contoural, Elemental, Iterable, Lengthsome, Multipolygonal,
    MultipolygonalIntoIteratorPolygon, Multisegmental,
    MultisegmentalIndexSegment, Multivertexal, MultivertexalIndexVertex,
    Polygonal, PolygonalContour, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental,
};

use super::event::Event;
use super::{linear, mixed, segment, shaped};

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

pub(crate) fn relate_to_multipolygon<
    Border,
    Multipolygon,
    Point: Clone + Ord,
    Polygon,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    polygon: &Polygon,
    multipolygonal: &Multipolygon,
) -> Relation
where
    shaped::Operation<Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Border:
        Bounded<&'a Scalar> + Contoural<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multipolygon:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    for<'a> &'a Polygon: Bounded<&'a Scalar>
        + Polygonal<Contour = &'a Border, IntoIteratorHole = &'a Border>,
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
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Border>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Border>: Elemental,
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
    let polygon_bounding_box = polygon.to_bounding_box();
    let multipolygonal_bounding_box = multipolygonal.to_bounding_box();
    if polygon_bounding_box.disjoint_with(&multipolygonal_bounding_box) {
        return Relation::Disjoint;
    }
    let multipolygonal_polygons = multipolygonal.polygons();
    let multipolygonal_bounding_boxes = multipolygonal_polygons
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let intersecting_polygons_ids = to_boxes_ids_with_intersection(
        &multipolygonal_bounding_boxes,
        &polygon_bounding_box,
    );
    if intersecting_polygons_ids.is_empty() {
        return Relation::Disjoint;
    }
    let min_max_x = polygon_bounding_box.get_max_x().min(unsafe {
        intersecting_polygons_ids
            .iter()
            .map(|&index| multipolygonal_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    });
    let border_segments = polygon.border().segments();
    let intersecting_holes_segments = polygon
        .holes()
        .into_iter()
        .filter_map(|hole| {
            if hole
                .to_bounding_box()
                .disjoint_with(&multipolygonal_bounding_box)
            {
                None
            } else {
                Some(hole.segments())
            }
        })
        .collect::<Vec<_>>();
    let intersecting_polygons_borders_segments = intersecting_polygons_ids
        .iter()
        .map(|&polygon_id| {
            multipolygonal_polygons[polygon_id].border().segments()
        })
        .collect::<Vec<_>>();
    let intersecting_polygons_holes_segments = intersecting_polygons_ids
        .iter()
        .flat_map(|&polygon_id| {
            multipolygonal_polygons[polygon_id]
                .holes()
                .into_iter()
                .filter_map(|hole| {
                    if hole
                        .to_bounding_box()
                        .disjoint_with(&polygon_bounding_box)
                    {
                        None
                    } else {
                        Some(hole.segments())
                    }
                })
        })
        .collect::<Vec<_>>();
    shaped::Operation::<Point>::from_segments_iterators(
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
            intersecting_polygons_borders_segments
                .iter()
                .map(|segments| segments.len())
                .sum::<usize>()
                + intersecting_polygons_holes_segments
                    .iter()
                    .map(|segments| segments.len())
                    .sum::<usize>(),
            intersecting_polygons_borders_segments
                .into_iter()
                .flat_map(|border_segments| {
                    border_segments.into_iter().cloned()
                })
                .chain(
                    intersecting_polygons_holes_segments.into_iter().flat_map(
                        |hole_segments| hole_segments.into_iter().cloned(),
                    ),
                ),
        ),
    )
    .into_relation(
        true,
        intersecting_polygons_ids.len() == multipolygonal_polygons.len(),
        min_max_x,
    )
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
