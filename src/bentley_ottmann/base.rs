use core::convert::From;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::operations::relate_segments;
use crate::relatable::Relation;
use crate::traits::{Point, Segment};
use std::cmp::Ordering;

use super::event::is_left_event;
use super::events_registry::EventsRegistry;
use super::traits::{EventsQueue, SweepLine};

pub(crate) fn sweep<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
    Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    Segment: From<(Endpoint, Endpoint)> + self::Segment<Scalar, Point = Endpoint>,
>(
    segments: &[Segment],
) -> Vec<Segment> {
    let mut result = Vec::with_capacity(segments.len());
    let mut events_registry = EventsRegistry::from(segments);
    if let Some(event) = events_registry.pop() {
        debug_assert!(is_left_event(event));
        let mut start = events_registry.get_event_start(event).clone();
        let mut segments_ids_containing_start =
            Vec::from([events_registry.to_left_event_segment_id(event)]);
        events_registry.insert(event);
        while let Some(event) = events_registry.pop() {
            let left_event = if is_left_event(event) {
                if let Some(equal_segment_event) = events_registry.find(event) {
                    events_registry.merge_equal_segment_events(equal_segment_event, event);
                } else {
                    events_registry.insert(event);
                    if let Some(below_event) = events_registry.below(event) {
                        events_registry.detect_intersection(below_event, event);
                    }
                    if let Some(above_event) = events_registry.above(event) {
                        events_registry.detect_intersection(event, above_event);
                    }
                }
                event
            } else {
                let event_opposite = events_registry.get_opposite(event);
                debug_assert!(is_left_event(event_opposite));
                if let Some(equal_segment_event) = events_registry.find(event_opposite) {
                    let (maybe_above_event, maybe_below_event) = (
                        events_registry.above(equal_segment_event),
                        events_registry.below(equal_segment_event),
                    );
                    events_registry.remove(equal_segment_event);
                    if let (Some(above_event), Some(below_event)) =
                        (maybe_above_event, maybe_below_event)
                    {
                        events_registry.detect_intersection(below_event, above_event);
                    }
                    if equal_segment_event != event_opposite {
                        events_registry
                            .merge_equal_segment_events(event_opposite, equal_segment_event);
                    }
                    result.push(Segment::from((
                        events_registry.get_event_start(equal_segment_event).clone(),
                        events_registry.get_event_end(equal_segment_event).clone(),
                    )));
                }
                event_opposite
            };
            if start.ne(events_registry.get_event_start(event)) {
                for (index, &first_segment_id) in segments_ids_containing_start
                    [..segments_ids_containing_start.len() - 1]
                    .iter()
                    .enumerate()
                {
                    let first_min_collinear_segment_id =
                        events_registry.to_min_collinear_segment_id(first_segment_id);
                    let first_start = events_registry.get_segment_start(first_segment_id);
                    let first_end = events_registry.get_segment_end(first_segment_id);
                    for &second_segment_id in &segments_ids_containing_start[index + 1..] {
                        let second_start = events_registry.get_segment_start(second_segment_id);
                        let second_end = events_registry.get_segment_end(second_segment_id);
                        let relation = if first_segment_id == second_segment_id {
                            Relation::Equal
                        } else if first_min_collinear_segment_id
                            != events_registry.to_min_collinear_segment_id(second_segment_id)
                        {
                            if first_start == second_end || second_start == first_end {
                                Relation::Touch
                            } else {
                                Relation::Cross
                            }
                        } else if first_start.max(second_start).eq(first_end.min(second_end)) {
                            Relation::Touch
                        } else {
                            match first_start.cmp(second_start) {
                                Ordering::Equal => match first_end.cmp(second_end) {
                                    Ordering::Equal => Relation::Equal,
                                    Ordering::Greater => Relation::Composite,
                                    Ordering::Less => Relation::Component,
                                },
                                Ordering::Greater => match first_end.cmp(second_end) {
                                    Ordering::Greater => Relation::Overlap,
                                    _ => Relation::Component,
                                },
                                Ordering::Less => match first_end.cmp(second_end) {
                                    Ordering::Less => Relation::Overlap,
                                    _ => Relation::Composite,
                                },
                            }
                        };
                        debug_assert_eq!(
                            relation,
                            relate_segments(first_start, first_end, second_start, second_end)
                        );
                    }
                }
                start = events_registry.get_event_start(event).clone();
                segments_ids_containing_start.clear();
            }
            segments_ids_containing_start
                .push(events_registry.to_left_event_segment_id(left_event));
        }
    }
    result
}
