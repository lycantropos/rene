use core::convert::From;

use crate::constants::{MIN_CONTOUR_VERTICES_COUNT, MIN_MULTISEGMENT_SEGMENTS_COUNT};
use crate::contracts::are_contour_vertices_non_degenerate;
use crate::operations::Orient;
use crate::relatable::Relation;
use crate::traits::{Contour, Multisegment, Segment};

use super::event::{is_left_event, Event};
use super::events_registry::EventsRegistry;
use super::sweep::{Intersection, Sweep};

pub(crate) fn is_contour_valid<
    Endpoint: Ord + Orient,
    Segment: self::Segment<Point = Endpoint>,
    Contour: self::Contour<Point = Endpoint, Segment = Segment>,
>(
    contour: &Contour,
) -> bool
where
    for<'a> Sweep<Endpoint>: From<&'a [Segment]> + Iterator<Item = Intersection<Endpoint>>,
{
    are_contour_vertices_non_degenerate(&contour.vertices()) && {
        let segments = contour.segments();
        segments.len() >= MIN_CONTOUR_VERTICES_COUNT
            && segments
                .iter()
                .all(|segment| segment.start() != segment.end())
            && {
                let mut sweep = Sweep::from(segments.as_slice());
                let mut neighbour_segments_touches_count = 0usize;
                while let Some(intersection) = sweep.next() {
                    debug_assert_eq!(
                        intersection.start == intersection.end,
                        matches!(intersection.relation, Relation::Touch | Relation::Cross)
                    );
                    let touches_at_vertices = intersection.relation == Relation::Touch
                        && (intersection
                            .start
                            .eq(sweep.get_segment_start(intersection.first_segment_id))
                            || intersection
                                .start
                                .eq(sweep.get_segment_end(intersection.first_segment_id)))
                        && (intersection
                            .start
                            .eq(sweep.get_segment_start(intersection.second_segment_id))
                            || intersection
                                .start
                                .eq(sweep.get_segment_end(intersection.second_segment_id)));
                    let neighbour_segments_intersection = intersection
                        .first_segment_id
                        .abs_diff(intersection.second_segment_id)
                        == 1
                        || (intersection.first_segment_id == segments.len() - 1
                            && intersection.second_segment_id == 0)
                        || (intersection.second_segment_id == segments.len() - 1
                            && intersection.first_segment_id == 0);
                    if !(touches_at_vertices && neighbour_segments_intersection) {
                        return false;
                    }
                    neighbour_segments_touches_count += 1;
                }
                neighbour_segments_touches_count == segments.len()
            }
    }
}

pub(crate) fn is_multisegment_valid<
    Endpoint: PartialEq,
    Segment: self::Segment<Point = Endpoint>,
    Multisegment: self::Multisegment<Point = Endpoint, Segment = Segment>,
>(
    multisegment: &Multisegment,
) -> bool
where
    for<'a> Sweep<Endpoint>: From<&'a [Segment]> + Iterator<Item = Intersection<Endpoint>>,
{
    let segments = multisegment.segments();
    segments.len() >= MIN_MULTISEGMENT_SEGMENTS_COUNT
        && segments
            .iter()
            .all(|segment| segment.start() != segment.end())
        && Sweep::from(segments.as_slice())
            .all(|intersection| intersection.relation == Relation::Touch)
}

pub(crate) fn to_unique_non_crossing_or_overlapping_segments<
    Scalar,
    Endpoint: Clone,
    Segment: From<(Endpoint, Endpoint)> + self::Segment<Point = Endpoint>,
>(
    segments: &[Segment],
) -> Vec<Segment>
where
    for<'a> EventsRegistry<Endpoint, true>: From<&'a [Segment]> + Iterator<Item = Event>,
{
    let mut result = Vec::with_capacity(segments.len());
    let mut events_registry = EventsRegistry::<Endpoint, true>::from(segments);
    while let Some(event) = events_registry.next() {
        if !is_left_event(event) {
            result.push(Segment::from((
                events_registry.get_event_start(event).clone(),
                events_registry.get_event_end(event).clone(),
            )))
        }
    }
    result
}
