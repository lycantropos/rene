use core::convert::From;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::traits::{Point, Segment};

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
    while let Some(event) = events_registry.pop() {
        if is_left_event(event) {
            if events_registry.find(event).is_none() {
                events_registry.insert(event);
                if let Some(below_event) = events_registry.below(event) {
                    events_registry.detect_intersection(below_event, event);
                }
                if let Some(above_event) = events_registry.above(event) {
                    events_registry.detect_intersection(event, above_event);
                }
            }
        } else {
            let event = events_registry.get_opposite(event);
            if let Some(equal_segment_event) = events_registry.find(event) {
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
                result.push(Segment::from((
                    events_registry.get_event_start(equal_segment_event).clone(),
                    events_registry.get_event_end(equal_segment_event).clone(),
                )));
            }
        }
    }
    result
}
