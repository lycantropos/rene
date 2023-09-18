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
use crate::relating::mixed;
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, Multivertexal, MultivertexalIndexVertex,
    Polygonal, PolygonalIntoIteratorHole, Segmental,
};

use super::event::Event;
use super::linear;
use super::segment;

pub(super) fn relate_to_multisegmental<
    const FIRST_IS_CONTOUR: bool,
    const SECOND_IS_CONTOUR: bool,
    First,
    Second,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    first: &First,
    second: &Second,
) -> Relation
where
    for<'a> &'a Output: Signed,
    for<'a> &'a First:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Second:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    let first_bounding_box = first.to_bounding_box();
    let second_bounding_box = second.to_bounding_box();
    if first_bounding_box.disjoint_with(&second_bounding_box) {
        return Relation::Disjoint;
    }
    let first_segments = first.segments();
    let first_bounding_boxes = first_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let first_intersecting_segments_ids = to_boxes_ids_with_intersection(
        &first_bounding_boxes,
        &second_bounding_box,
    );
    if first_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    }
    let second_segments = second.segments();
    if first_intersecting_segments_ids.len() == 1 {
        let first_intersecting_segment =
            &first_segments[first_intersecting_segments_ids[0]];
        return match if SECOND_IS_CONTOUR {
            segment::relate_to_contour_segments(
                first_intersecting_segment,
                second_segments.iter(),
            )
        } else {
            segment::relate_to_multisegment_segments(
                first_intersecting_segment,
                second_segments.iter(),
            )
        } {
            Relation::Component | Relation::Equal => Relation::Overlap,
            relation => relation,
        };
    }
    let second_bounding_boxes = second_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let second_intersecting_segments_ids = to_boxes_ids_with_intersection(
        &second_bounding_boxes,
        &first_bounding_box,
    );
    if second_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    } else if second_intersecting_segments_ids.len() == 1 {
        let second_intersecting_segment =
            &second_segments[second_intersecting_segments_ids[0]];
        return match if FIRST_IS_CONTOUR {
            segment::relate_to_contour_segments(
                second_intersecting_segment,
                first_intersecting_segments_ids
                    .iter()
                    .map(|&index| &first_segments[index]),
            )
        } else {
            segment::relate_to_multisegment_segments(
                second_intersecting_segment,
                first_intersecting_segments_ids
                    .iter()
                    .map(|&index| &first_segments[index]),
            )
        } {
            Relation::Component | Relation::Equal => Relation::Overlap,
            Relation::Composite
                if first_intersecting_segments_ids.len()
                    != first_segments.len() =>
            {
                Relation::Overlap
            }
            relation => relation.to_complement(),
        };
    }
    let min_max_x = unsafe {
        first_intersecting_segments_ids
            .iter()
            .map(|&index| first_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    }
    .min(unsafe {
        second_intersecting_segments_ids
            .iter()
            .map(|&index| second_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    });
    let max_min_x = unsafe {
        first_intersecting_segments_ids
            .iter()
            .map(|&index| first_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    }
    .max(unsafe {
        second_intersecting_segments_ids
            .iter()
            .map(|&index| second_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    });
    let first_intersecting_segments = first_intersecting_segments_ids
        .iter()
        .filter_map(|&index| {
            if max_min_x <= first_bounding_boxes[index].get_max_x()
                && first_bounding_boxes[index].get_min_x() <= min_max_x
            {
                Some(&first_segments[index])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if first_intersecting_segments.is_empty() {
        return Relation::Disjoint;
    }
    let second_intersecting_segments = second_intersecting_segments_ids
        .iter()
        .filter_map(|&index| {
            if max_min_x <= second_bounding_boxes[index].get_max_x()
                && second_bounding_boxes[index].get_min_x() <= min_max_x
            {
                Some(&second_segments[index])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    debug_assert!(!second_intersecting_segments.is_empty());
    linear::Operation::from((
        &first_intersecting_segments,
        &second_intersecting_segments,
    ))
    .into_relation(
        first_intersecting_segments.len() == first_segments.len(),
        second_intersecting_segments.len() == second_segments.len(),
        min_max_x,
    )
}

pub(super) fn relate_to_polygon<
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
    multisegment: &Multisegment,
    polygon: &Polygon,
) -> Relation
where
    mixed::Operation<true, Point>:
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
    for<'a> &'a Polygon: Polygonal<Contour = &'a Border, IndexHole = Border>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    let multisegmental_bounding_box = multisegment.to_bounding_box();
    let border = polygon.border();
    let polygon_bounding_box = border.to_bounding_box();
    if multisegmental_bounding_box.disjoint_with(&polygon_bounding_box) {
        return Relation::Disjoint;
    }
    let multisegmental_segments = multisegment.segments();
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
        let intersecting_segment =
            &multisegmental_segments[intersecting_segments_ids[0]];
        let relation =
            segment::relate_to_polygon(intersecting_segment, polygon);
        return if multisegmental_segments.len()
            == intersecting_segments_ids.len()
        {
            relation
        } else {
            match relation {
                Relation::Component => Relation::Touch,
                Relation::Enclosed | Relation::Within => Relation::Cross,
                relation => relation,
            }
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
    let border_segments = border.segments();
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
    mixed::Operation::<true, Point>::from_segments_iterators(
        (
            intersecting_segments.len(),
            intersecting_segments.iter().copied().cloned(),
        ),
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
    )
    .into_relation(
        intersecting_segments.len() == multisegmental_segments.len(),
        min_max_x,
    )
}
