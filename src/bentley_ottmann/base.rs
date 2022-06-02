use core::convert::From;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Parity, Signed};

use crate::traits::{Point, Segment};

use super::events_queue::EventsQueue;
use super::sweep_line::SweepLine;

pub(crate) fn sweep<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
    Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    Segment: From<(Endpoint, Endpoint)> + self::Segment<Scalar, Point = Endpoint>,
>(
    segments: &[Segment],
) -> Vec<Segment> {
    let mut result = Vec::with_capacity(segments.len());
    let mut endpoints = Vec::with_capacity(2 * segments.len());
    let mut opposites = Vec::with_capacity(2 * segments.len());
    let mut events_queue = EventsQueue::new(&mut endpoints, &mut opposites, segments);
    let mut sweep_line = SweepLine::new(&endpoints, &opposites);
    while let Some(event_index) = events_queue.pop() {
        if event_index.is_even() {
            // left endpoint event
            if let None = sweep_line.find(event_index) {
                sweep_line.insert(event_index);
                if let Some(below_event_index) = sweep_line.below(event_index) {
                    events_queue.detect_intersection(
                        below_event_index,
                        event_index,
                        &mut sweep_line,
                    );
                }
                if let Some(above_event_index) = sweep_line.above(event_index) {
                    events_queue.detect_intersection(
                        event_index,
                        above_event_index,
                        &mut sweep_line,
                    );
                }
            }
        } else {
            let event_index = opposites[event_index];
            if let Some(equal_segment_event_index) = sweep_line.find(event_index) {
                let (maybe_above_event, maybe_below_event_index) = (
                    sweep_line.above(equal_segment_event_index),
                    sweep_line.below(equal_segment_event_index),
                );
                sweep_line.remove(equal_segment_event_index);
                match (maybe_above_event, maybe_below_event_index) {
                    (Some(above_event_index), Some(below_event_index)) => {
                        events_queue.detect_intersection(
                            below_event_index,
                            above_event_index,
                            &mut sweep_line,
                        );
                    }
                    _ => {}
                }
                result.push(Segment::from((
                    events_queue
                        .get_event_start(equal_segment_event_index)
                        .clone(),
                    events_queue
                        .get_event_end(equal_segment_event_index)
                        .clone(),
                )));
            }
        }
    }
    result
}
