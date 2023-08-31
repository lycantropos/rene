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
    Elemental, Iterable, Lengthsome, Multisegmental, Segmental,
};

use super::event::Event;
use super::linear::Operation;
use super::segment::{
    relate_to_contour_segments, relate_to_multisegment_segments,
};

pub(crate) fn relate_to_multisegmental<
    const FIRST_IS_CONTOUR: bool,
    const SECOND_IS_CONTOUR: bool,
    First,
    Second,
    Point: Clone + Hash + Ord,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
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
    for<'a, 'b> Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
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
        let (first_intersecting_segment_start, first_intersecting_segment_end) =
            first_segments[first_intersecting_segments_ids[0]].endpoints();
        return match if SECOND_IS_CONTOUR {
            relate_to_contour_segments(
                first_intersecting_segment_start,
                first_intersecting_segment_end,
                second_segments.iter(),
            )
        } else {
            relate_to_multisegment_segments(
                first_intersecting_segment_start,
                first_intersecting_segment_end,
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
        let (
            second_intersecting_segment_start,
            second_intersecting_segment_end,
        ) = second_segments[second_intersecting_segments_ids[0]].endpoints();
        return match if FIRST_IS_CONTOUR {
            relate_to_contour_segments(
                second_intersecting_segment_start,
                second_intersecting_segment_end,
                first_intersecting_segments_ids
                    .iter()
                    .map(|&index| &first_segments[index]),
            )
        } else {
            relate_to_multisegment_segments(
                second_intersecting_segment_start,
                second_intersecting_segment_end,
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
    Operation::from((
        &first_intersecting_segments,
        &second_intersecting_segments,
    ))
    .into_relation(
        first_intersecting_segments.len() == first_segments.len(),
        second_intersecting_segments.len() == second_segments.len(),
        min_max_x,
    )
}
