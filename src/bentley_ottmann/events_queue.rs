use core::convert::From;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Parity, Signed};

use crate::operations::{intersect_crossing_segments, orient, to_sorted_pair};
use crate::oriented::Orientation;
use crate::traits;
use crate::traits::{Point, Segment};

use super::event::Event;
use super::events_queue_key::EventsQueueKey;
use super::sweep_line::SweepLine;

pub(super) struct EventsQueue<Scalar, Endpoint> {
    queue: BinaryHeap<Reverse<EventsQueueKey<Endpoint>>>,
    endpoints: *mut Vec<Endpoint>,
    opposites: *mut Vec<Event>,
    _phantom: PhantomData<fn() -> Scalar>,
}

impl<Scalar, Endpoint: Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn endpoints(&self) -> &mut Vec<Endpoint> {
        unsafe { &mut *self.endpoints }
    }

    pub(super) fn opposites(&self) -> &mut Vec<Event> {
        unsafe { &mut *self.opposites }
    }

    pub(super) fn get_event_start(&self, event: Event) -> &Endpoint {
        &self.endpoints()[event]
    }

    pub(super) fn get_event_end(&self, event: Event) -> &Endpoint {
        &self.endpoints()[self.get_opposite(event)]
    }

    pub(super) fn get_opposite(&self, event: Event) -> Event {
        self.opposites()[event]
    }
}

impl<Scalar, Endpoint: Clone + Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn new<Segment: self::Segment<Scalar, Point = Endpoint>>(
        endpoints: &mut Vec<Endpoint>,
        opposites: &mut Vec<Event>,
        segments: &[Segment],
    ) -> Self {
        let mut result = Self {
            queue: BinaryHeap::with_capacity(endpoints.capacity()),
            endpoints,
            opposites,
            _phantom: PhantomData,
        };
        for segment in segments {
            let (start, end) = to_sorted_pair((segment.start(), segment.end()));
            let left_event = result.endpoints().len();
            result.endpoints().push(start.clone());
            let right_event = result.endpoints().len();
            result.endpoints().push(end.clone());
            result.opposites().push(right_event);
            result.opposites().push(left_event);
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
                        self.divide_below_event_by_midpoint(below_event, point.clone());
                        self.divide_event_by_midpoint(event, point, sweep_line);
                    }
                } else if below_event_start_orientation != Orientation::Collinear {
                    if event_start < below_event_end && below_event_end < event_end {
                        let point = below_event_end.clone();
                        self.divide_event_by_midpoint(event, point, sweep_line);
                    }
                } else if event_start < below_event_start && below_event_start < event_end {
                    let point = below_event_start.clone();
                    self.divide_event_by_midpoint(event, point, sweep_line);
                }
            }
        } else if event_end_orientation != Orientation::Collinear {
            if below_event_start < event_start && event_start < below_event_end {
                let point = event_start.clone();
                self.divide_below_event_by_midpoint(below_event, point);
            }
        } else if event_start_orientation != Orientation::Collinear {
            if below_event_start < event_end && event_end < below_event_end {
                let point = event_end.clone();
                self.divide_below_event_by_midpoint(below_event, point);
            }
        } else if event_start == below_event_start {
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
                self.divide_composite_event_by_inner_points(below_event, min_point, max_point);
            } else {
                let (max_start, min_end) = (event_start.clone(), below_event_end.clone());
                self.divide_overlapping_events(below_event, event, max_start, min_end);
            }
        } else if event_start < below_event_start && below_event_start < event_end {
            if below_event_end < event_end {
                let min_point = below_event_start.clone();
                let max_point = below_event_end.clone();
                self.divide_composite_event_by_inner_points(event, min_point, max_point);
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
        let (min_end_max_start_event, min_end_max_end_event) =
            self.divide(max_start_event, min_end);
        self.push(min_end_max_start_event);
        self.push(min_end_max_end_event);
        let (max_start_min_start_event, _) = self.divide(min_start_event, max_start);
        self.push(max_start_min_start_event);
    }

    fn divide_composite_event_by_inner_points(
        &mut self,
        event: Event,
        min_point: Endpoint,
        max_point: Endpoint,
    ) {
        let (max_point_event_start_index, max_point_event_end_index) =
            self.divide(event, max_point);
        self.push(max_point_event_start_index);
        self.push(max_point_event_end_index);
        let (min_point_event_start_index, _) = self.divide(event, min_point);
        self.push(min_point_event_start_index);
    }

    fn divide_below_event_by_midpoint(&mut self, below_event: Event, point: Endpoint) {
        let (point_below_event_start_index, point_below_event_end_index) =
            self.divide(below_event, point);
        self.push(point_below_event_start_index);
        self.push(point_below_event_end_index);
    }

    fn divide_event_by_midpoint(
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
}

impl<Scalar, Endpoint: Clone + self::Point<Scalar> + Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn divide(&mut self, event: Event, mid_point: Endpoint) -> (Event, Event) {
        debug_assert!(event.is_even());
        let mid_point_to_event_end_event = self.endpoints().len();
        self.endpoints().push(mid_point.clone());
        self.opposites().push(self.opposites()[event]);
        self.opposites()[self.get_opposite(event)] = mid_point_to_event_end_event;
        let mid_point_to_event_start_event = self.endpoints().len();
        self.endpoints().push(mid_point.clone());
        self.opposites().push(event);
        self.opposites()[event] = mid_point_to_event_start_event;
        (mid_point_to_event_start_event, mid_point_to_event_end_event)
    }
}

impl<Scalar, Endpoint: Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn pop(&mut self) -> Option<Event> {
        self.queue.pop().map(|key| key.0.event)
    }

    fn push(&mut self, event: Event) {
        let key = Reverse(EventsQueueKey::new(
            event,
            self.endpoints(),
            self.opposites(),
        ));
        self.queue.push(key)
    }
}
