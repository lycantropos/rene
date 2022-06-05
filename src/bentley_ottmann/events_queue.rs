use core::convert::From;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::operations::{intersect_crossing_segments, orient, to_sorted_pair};
use crate::oriented::Orientation;
use crate::traits;
use crate::traits::{Point, Segment};

use super::event::{is_left_event, Event};
use super::events_queue_key::EventsQueueKey;
use super::sweep_line::SweepLine;

pub(super) struct EventsQueue<Scalar, Endpoint> {
    endpoints: Vec<Endpoint>,
    opposites: Vec<Event>,
    queue: BinaryHeap<Reverse<EventsQueueKey<Endpoint>>>,
    _phantom: PhantomData<fn() -> Scalar>,
}

impl<Scalar, Endpoint> EventsQueue<Scalar, Endpoint> {
    pub(super) fn get_endpoints(&self) -> &Vec<Endpoint> {
        &self.endpoints
    }

    pub(super) fn get_event_start(&self, event: Event) -> &Endpoint {
        &self.endpoints[event]
    }

    pub(super) fn get_event_end(&self, event: Event) -> &Endpoint {
        &self.endpoints[self.get_opposite(event)]
    }

    pub(super) fn get_opposite(&self, event: Event) -> Event {
        self.opposites[event]
    }

    pub(super) fn get_opposites(&self) -> &Vec<Event> {
        &self.opposites
    }
}

impl<Scalar, Endpoint: Ord, Segment: self::Segment<Scalar, Point = Endpoint>> From<&[Segment]>
    for EventsQueue<Scalar, Endpoint>
{
    fn from(segments: &[Segment]) -> Self {
        let capacity = 2 * segments.len();
        let mut result = Self {
            endpoints: Vec::with_capacity(capacity),
            opposites: Vec::with_capacity(capacity),
            queue: BinaryHeap::with_capacity(capacity),
            _phantom: PhantomData,
        };
        for (index, segment) in segments.iter().enumerate() {
            let (start, end) = to_sorted_pair((segment.start(), segment.end()));
            let left_event = 2 * index;
            let right_event = 2 * index + 1;
            result.endpoints.push(start);
            result.endpoints.push(end);
            result.opposites.push(right_event);
            result.opposites.push(left_event);
            result.push(left_event);
            result.push(right_event);
        }
        result
    }
}

impl<
        Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + From<(Scalar, Scalar)> + Ord + traits::Point<Scalar>,
    > EventsQueue<Scalar, Endpoint>
{
    pub(super) fn detect_intersection(
        &mut self,
        below_event: Event,
        event: Event,
        sweep_line: &mut SweepLine<Scalar, Endpoint>,
    ) {
        let event_start = self.get_event_start(event);
        let event_end = self.get_event_end(event);
        let below_event_start = self.get_event_start(below_event);
        let below_event_end = self.get_event_end(below_event);

        let event_start_orientation = orient(below_event_end, below_event_start, event_start);
        let event_end_orientation = orient(below_event_end, below_event_start, event_end);
        if event_start_orientation != Orientation::Collinear
            && event_end_orientation != Orientation::Collinear
        {
            if event_start_orientation != event_end_orientation {
                let below_event_start_orientation =
                    orient(event_start, event_end, below_event_start);
                let below_event_end_orientation = orient(event_start, event_end, below_event_end);
                if below_event_start_orientation != Orientation::Collinear
                    && below_event_end_orientation != Orientation::Collinear
                {
                    if below_event_start_orientation != below_event_end_orientation {
                        let point = intersect_crossing_segments(
                            event_start,
                            event_end,
                            below_event_start,
                            below_event_end,
                        );
                        self.divide_event_by_midpoint(below_event, point.clone());
                        self.divide_event_by_midpoint_checking_above(event, point, sweep_line);
                    }
                } else if below_event_start_orientation != Orientation::Collinear {
                    if event_start < below_event_end && below_event_end < event_end {
                        let point = below_event_end.clone();
                        self.divide_event_by_midpoint_checking_above(event, point, sweep_line);
                    }
                } else if event_start < below_event_start && below_event_start < event_end {
                    let point = below_event_start.clone();
                    self.divide_event_by_midpoint_checking_above(event, point, sweep_line);
                }
            }
        } else if event_end_orientation != Orientation::Collinear {
            if below_event_start < event_start && event_start < below_event_end {
                let point = event_start.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else if event_start_orientation != Orientation::Collinear {
            if below_event_start < event_end && event_end < below_event_end {
                let point = event_end.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else if event_start == below_event_start {
            // segments share the right endpoint
            debug_assert!(event_end != below_event_end);
            let (max_end_event, min_end_event) = if event_end < below_event_end {
                (below_event, event)
            } else {
                (event, below_event)
            };
            sweep_line.remove(max_end_event);
            let min_end = self.get_event_end(min_end_event).clone();
            let (_, min_end_max_end_event) = self.divide(max_end_event, min_end);
            self.push(min_end_max_end_event);
        } else if event_end == below_event_end {
            // segments share the right endpoint
            let (max_start_event, min_start_event) = if event_start < below_event_start {
                (below_event, event)
            } else {
                (event, below_event)
            };
            let max_start = self.get_event_start(max_start_event).clone();
            let (max_start_min_start_event, _) = self.divide(min_start_event, max_start);
            self.push(max_start_min_start_event);
        } else if below_event_start < event_start && event_start < below_event_end {
            if event_end < below_event_end {
                let (max_point, min_point) = (event_end.clone(), event_start.clone());
                self.divide_event_by_midpoints(below_event, min_point, max_point);
            } else {
                let (max_start, min_end) = (event_start.clone(), below_event_end.clone());
                self.divide_overlapping_events(below_event, event, max_start, min_end);
            }
        } else if event_start < below_event_start && below_event_start < event_end {
            if below_event_end < event_end {
                let min_point = below_event_start.clone();
                let max_point = below_event_end.clone();
                self.divide_event_by_midpoints(event, min_point, max_point);
            } else {
                let max_start = below_event_start.clone();
                let min_end = event_end.clone();
                self.divide_overlapping_events(event, below_event, max_start, min_end);
            }
        }
    }

    fn divide_overlapping_events(
        &mut self,
        min_start_event: Event,
        max_start_event: Event,
        max_start: Endpoint,
        min_end: Endpoint,
    ) {
        self.divide_event_by_midpoint(max_start_event, min_end);
        let (max_start_min_start_event, _) = self.divide(min_start_event, max_start);
        self.push(max_start_min_start_event);
    }

    fn divide_event_by_midpoint(&mut self, event: Event, point: Endpoint) {
        let (point_to_event_start_index, point_to_event_end_index) = self.divide(event, point);
        self.push(point_to_event_start_index);
        self.push(point_to_event_end_index);
    }

    fn divide_event_by_midpoint_checking_above(
        &mut self,
        event: Event,
        point: Endpoint,
        sweep_line: &mut SweepLine<Scalar, Endpoint>,
    ) {
        if let Some(above_event) = sweep_line.above(event) {
            if self
                .get_event_start(above_event)
                .eq(self.get_event_start(event))
                && self.get_event_end(above_event).eq(&point)
            {
                sweep_line.remove(above_event);
            }
        }
        let (point_event_start_event, point_event_end_event) = self.divide(event, point);
        self.push(point_event_start_event);
        self.push(point_event_end_event);
    }

    fn divide_event_by_midpoints(
        &mut self,
        event: Event,
        min_midpoint: Endpoint,
        max_midpoint: Endpoint,
    ) {
        self.divide_event_by_midpoint(event, max_midpoint);
        let (min_midpoint_to_event_start_index, _) = self.divide(event, min_midpoint);
        self.push(min_midpoint_to_event_start_index);
    }
}

impl<Scalar, Endpoint: Clone + self::Point<Scalar> + Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn divide(&mut self, event: Event, mid_point: Endpoint) -> (Event, Event) {
        debug_assert!(is_left_event(event));
        let opposite_event = self.get_opposite(event);
        let mid_point_to_event_end_event = self.endpoints.len();
        self.endpoints.push(mid_point.clone());
        self.opposites.push(opposite_event);
        self.opposites[opposite_event] = mid_point_to_event_end_event;
        let mid_point_to_event_start_event = self.endpoints.len();
        self.endpoints.push(mid_point);
        self.opposites.push(event);
        self.opposites[event] = mid_point_to_event_start_event;
        (mid_point_to_event_start_event, mid_point_to_event_end_event)
    }
}

impl<Scalar, Endpoint: Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn pop(&mut self) -> Option<Event> {
        self.queue.pop().map(|key| key.0.event)
    }

    fn push(&mut self, event: Event) {
        self.queue.push(Reverse(EventsQueueKey::new(
            event,
            &self.endpoints,
            &self.opposites,
        )))
    }
}
