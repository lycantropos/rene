use core::convert::From;
use std::cmp::Ordering;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::operations::relate_segments;
use crate::relatable::Relation;
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
    let mut events_registry = EventsRegistry::<Scalar, Endpoint, true>::from(segments);
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
