use std::hash::Hash;
use std::ops::Div;

use crate::bounded::Bounded;
use crate::operations::{
    to_boxes_ids_with_intersection, CrossMultiply, IntersectCrossingSegments,
    Orient,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::{mixed, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multipolygonal,
    MultipolygonalIntoIteratorPolygon, Multisegmental,
    MultisegmentalIndexSegment, MultivertexalIndexVertex, Polygonal,
    PolygonalContour, PolygonalHoles, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental, Sequence,
};

use super::segment_endpoints;

pub(crate) fn relate_to_contour<
    'a,
    Contour,
    Point: 'a + Clone + PartialOrd,
    Segment: 'a,
>(
    segment: &'a Segment,
    contour: &'a Contour,
) -> Relation
where
    for<'b> &'b Contour: Multisegmental<IntoIteratorSegment = &'b Segment>,
    for<'b> &'b MultisegmentalIndexSegment<&'a Contour>: Segmental,
    for<'b> &'b Point: Orient,
    for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
{
    relate_to_contour_segments(segment, contour.segments().into_iter())
}

pub(crate) fn relate_to_multipolygon<
    Border,
    Multipolygon,
    Point: Clone + Ord,
    Polygon,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    segment: &Segment,
    multipolygon: &Multipolygon,
) -> Relation
where
    mixed::Operation<true, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Border:
        Bounded<&'a Scalar> + Contoural<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multipolygon: Multipolygonal<IndexPolygon = Polygon>,
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
    let segment_bounding_box = segment.to_bounding_box();
    let polygons = multipolygon.polygons();
    let polygons_bounding_boxes = polygons
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let intersecting_polygons_ids = to_boxes_ids_with_intersection(
        &polygons_bounding_boxes,
        &segment_bounding_box,
    );
    if intersecting_polygons_ids.is_empty() {
        Relation::Disjoint
    } else {
        let min_max_x = segment_bounding_box.get_max_x().min(unsafe {
            intersecting_polygons_ids
                .iter()
                .map(|&border_id| {
                    polygons_bounding_boxes[border_id].get_max_x()
                })
                .max()
                .unwrap_unchecked()
        });
        let intersecting_polygons_borders_segments = intersecting_polygons_ids
            .iter()
            .map(|&polygon_id| polygons[polygon_id].border().segments())
            .collect::<Vec<_>>();
        let intersecting_polygons_holes_segments = intersecting_polygons_ids
            .iter()
            .flat_map(|&polygon_id| {
                polygons[polygon_id].holes().into_iter().filter_map(|hole| {
                    if hole
                        .to_bounding_box()
                        .disjoint_with(&segment_bounding_box)
                    {
                        None
                    } else {
                        Some(hole.segments())
                    }
                })
            })
            .collect::<Vec<_>>();
        mixed::Operation::<true, Point>::from_segments_iterators(
            (1, std::iter::once(segment.clone())),
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
                        intersecting_polygons_holes_segments
                            .into_iter()
                            .flat_map(|hole_segments| {
                                hole_segments.into_iter().cloned()
                            }),
                    ),
            ),
        )
        .into_relation(true, min_max_x)
    }
}

pub(crate) fn relate_to_multisegment<
    'a,
    Multisegment,
    Point: 'a + Hash + Ord,
    Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd,
    Segment: 'a,
>(
    segment: &'a Segment,
    multisegment: &'a Multisegment,
) -> Relation
where
    &'a Multisegment: Multisegmental<IntoIteratorSegment = &'a Segment>,
    &'a Segment: Segmental<Endpoint = &'a Point>,
    for<'b> &'b MultisegmentalIndexSegment<&'a Multisegment>: Segmental,
    for<'b> &'b Point: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    relate_to_multisegment_segments(
        segment,
        multisegment.segments().into_iter(),
    )
}

pub(crate) fn relate_to_polygon<
    Border,
    Point: Clone + Ord,
    Polygon,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    segment: &Segment,
    polygon: &Polygon,
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
    let relation_without_holes =
        relate_to_region::<false, Border, Point, Scalar, Segment>(
            segment,
            polygon.border(),
        );
    if polygon.holes().len() > 0
        && matches!(
            relation_without_holes,
            Relation::Within | Relation::Enclosed
        )
    {
        let holes = polygon.holes();
        let relation_with_holes = if holes.len() == 1 {
            relate_to_region::<true, Border, Point, Scalar, Segment>(
                segment, &holes[0],
            )
        } else {
            relate_to_multiregion::<
                Border,
                PolygonalHoles<&Polygon>,
                Point,
                Scalar,
                Segment,
            >(segment, polygon.holes())
        };
        match relation_with_holes {
            Relation::Disjoint => relation_without_holes,
            Relation::Touch => Relation::Enclosed,
            Relation::Enclosed => Relation::Touch,
            Relation::Within => Relation::Disjoint,
            _ => relation_with_holes,
        }
    } else {
        relation_without_holes
    }
}

pub(crate) fn relate_to_segment<Point: PartialOrd, Segment>(
    first: &Segment,
    second: &Segment,
) -> Relation
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    segment_endpoints::relate_to_segment_endpoints(
        first.endpoints(),
        second.endpoints(),
    )
}

pub(super) fn relate_to_contour_segments<
    'a,
    Point: Clone + PartialOrd,
    Segment: 'a,
>(
    segment: &Segment,
    contour_segments: impl Iterator<Item = &'a Segment>,
) -> Relation
where
    for<'b> &'b Point: Orient,
    for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
{
    segment_endpoints::relate_to_contour_segments(
        segment.endpoints(),
        contour_segments,
    )
}

pub(super) fn relate_to_multiregion<
    Border,
    Borders: Sequence<IndexItem = Border>,
    Point: Clone + Ord,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    segment: &Segment,
    borders: Borders,
) -> Relation
where
    mixed::Operation<true, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Border>: Segmental,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Multisegmental<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    debug_assert!(borders.len() > 1);
    let segment_bounding_box = segment.to_bounding_box();
    let borders_bounding_boxes = borders
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let intersecting_borders_ids = to_boxes_ids_with_intersection(
        &borders_bounding_boxes,
        &segment_bounding_box,
    );
    if intersecting_borders_ids.is_empty() {
        Relation::Disjoint
    } else {
        let min_max_x = segment_bounding_box.get_max_x().min(unsafe {
            intersecting_borders_ids
                .iter()
                .map(|&border_id| {
                    borders_bounding_boxes[border_id].get_max_x()
                })
                .max()
                .unwrap_unchecked()
        });
        let intersecting_borders_segments = intersecting_borders_ids
            .iter()
            .map(|&border_id| borders[border_id].segments())
            .collect::<Vec<_>>();
        mixed::Operation::<true, Point>::from_segments_iterators(
            (1, std::iter::once(segment.clone())),
            (
                intersecting_borders_segments
                    .iter()
                    .map(|segments| segments.len())
                    .sum::<usize>(),
                intersecting_borders_segments.into_iter().flat_map(
                    |border_segments| border_segments.into_iter().cloned(),
                ),
            ),
        )
        .into_relation(
            intersecting_borders_ids.len() == borders.len(),
            min_max_x,
        )
    }
}

pub(super) fn relate_to_multisegment_segments<
    'a,
    Point: 'a + Hash + Ord,
    Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd,
    Segment: 'a,
>(
    segment: &'a Segment,
    multisegment_segments: impl Iterator<Item = &'a Segment>,
) -> Relation
where
    &'a Segment: Segmental<Endpoint = &'a Point>,
    for<'b> &'b Point: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    segment_endpoints::relate_to_multisegment_segments(
        segment.endpoints(),
        multisegment_segments,
    )
}

pub(super) fn relate_to_region<
    const REVERSE_ORIENTATION: bool,
    Border,
    Point: PartialOrd,
    Scalar: PartialOrd,
    Segment,
>(
    segment: &Segment,
    border: &Border,
) -> Relation
where
    for<'a> &'a Border: Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar> + Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    segment_endpoints::relate_to_region::<
        REVERSE_ORIENTATION,
        Border,
        Point,
        Scalar,
        Segment,
    >(segment.endpoints(), border)
}
