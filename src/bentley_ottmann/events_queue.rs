use core::convert::From;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Parity, Signed};

use crate::operations::{intersect_crossing_segments, orient, to_sorted_pair};
use crate::oriented::Orientation;
use crate::traits;
use crate::traits::{Point, Segment};

use super::events_queue_key::EventsQueueKey;
use super::sweep_line::SweepLine;

pub(super) struct EventsQueue<Scalar, Endpoint> {
    queue: BinaryHeap<Reverse<EventsQueueKey<Endpoint>>>,
    endpoints: *mut Vec<Endpoint>,
    opposites: *mut Vec<usize>,
    _phantom: PhantomData<fn() -> Scalar>,
}

impl<Scalar, Endpoint: Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn endpoints(&self) -> &mut Vec<Endpoint> {
        unsafe { &mut *self.endpoints }
    }

    pub(super) fn opposites(&self) -> &mut Vec<usize> {
        unsafe { &mut *self.opposites }
    }

    pub(super) fn get_event_start(&self, event_index: usize) -> &Endpoint {
        &self.endpoints()[event_index]
    }

    pub(super) fn get_event_end(&self, event_index: usize) -> &Endpoint {
        &self.endpoints()[self.get_opposite_index(event_index)]
    }

    pub(super) fn get_opposite_index(&self, event_index: usize) -> usize {
        self.opposites()[event_index]
    }
}

impl<Scalar, Endpoint: Clone + Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn new<Segment: self::Segment<Scalar, Point = Endpoint>>(
        endpoints: &mut Vec<Endpoint>,
        opposites: &mut Vec<usize>,
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
            let left_event_index = result.endpoints().len();
            result.endpoints().push(start.clone());
            let right_event_index = result.endpoints().len();
            result.endpoints().push(end.clone());
            result.opposites().push(right_event_index);
            result.opposites().push(left_event_index);
            result.push(left_event_index);
            result.push(right_event_index);
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
        below_event_index: usize,
        event_index: usize,
        sweep_line: &mut SweepLine<Scalar, Endpoint>,
    ) {
        let event_start = self.get_event_start(event_index);
        let event_end = self.get_event_end(event_index);
        let below_event_start = self.get_event_start(below_event_index);
        let below_event_end = self.get_event_end(below_event_index);

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
                        self.divide_below_event_by_midpoint(below_event_index, point.clone());
                        self.divide_event_by_midpoint(event_index, point, sweep_line);
                    }
                } else if below_event_start_orientation != Orientation::Collinear {
                    if event_start < below_event_end && below_event_end < event_end {
                        let point = below_event_end.clone();
                        self.divide_event_by_midpoint(event_index, point, sweep_line);
                    }
                } else if event_start < below_event_start && below_event_start < event_end {
                    let point = below_event_start.clone();
                    self.divide_event_by_midpoint(event_index, point, sweep_line);
                }
            }
        } else if event_end_orientation != Orientation::Collinear {
            if below_event_start < event_start && event_start < below_event_end {
                let point = event_start.clone();
                self.divide_below_event_by_midpoint(below_event_index, point);
            }
        } else if event_start_orientation != Orientation::Collinear {
            if below_event_start < event_end && event_end < below_event_end {
                let point = event_end.clone();
                self.divide_below_event_by_midpoint(below_event_index, point);
            }
        } else if event_start == below_event_start {
            let (max_end_event_index, min_end_event_index) = if event_end < below_event_end {
                (below_event_index, event_index)
            } else {
                (event_index, below_event_index)
            };
            sweep_line.remove(max_end_event_index);
            let min_end = self.get_event_end(min_end_event_index).clone();
            let (_, min_end_max_end_event_index) = self.divide(max_end_event_index, min_end);
            self.push(min_end_max_end_event_index);
        } else if event_end == below_event_end {
            // segments share the right endpoint
            let (max_start_event_index, min_start_event_index) = if event_start < below_event_start
            {
                (below_event_index, event_index)
            } else {
                (event_index, below_event_index)
            };
            let max_start = self.get_event_start(max_start_event_index).clone();
            let (max_start_min_start_event_index, _) =
                self.divide(min_start_event_index, max_start);
            self.push(max_start_min_start_event_index);
        } else if below_event_start < event_start && event_start < below_event_end {
            if event_end < below_event_end {
                let (max_point, min_point) = (event_end.clone(), event_start.clone());
                self.divide_composite_event_by_inner_points(
                    below_event_index,
                    min_point,
                    max_point,
                );
            } else {
                let (max_start, min_end) = (event_start.clone(), below_event_end.clone());
                self.divide_overlapping_events(below_event_index, event_index, max_start, min_end);
            }
        } else if event_start < below_event_start && below_event_start < event_end {
            if below_event_end < event_end {
                let min_point = below_event_start.clone();
                let max_point = below_event_end.clone();
                self.divide_composite_event_by_inner_points(event_index, min_point, max_point);
            } else {
                let max_start = below_event_start.clone();
                let min_end = event_end.clone();
                self.divide_overlapping_events(event_index, below_event_index, max_start, min_end);
            }
        }
    }

    fn divide_overlapping_events(
        &mut self,
        min_start_event_index: usize,
        max_start_event_index: usize,
        max_start: Endpoint,
        min_end: Endpoint,
    ) {
        let (min_end_max_start_event_index, min_end_max_end_event_index) =
            self.divide(max_start_event_index, min_end);
        self.push(min_end_max_start_event_index);
        self.push(min_end_max_end_event_index);
        let (max_start_min_start_event_index, _) = self.divide(min_start_event_index, max_start);
        self.push(max_start_min_start_event_index);
    }

    fn divide_composite_event_by_inner_points(
        &mut self,
        event_index: usize,
        min_point: Endpoint,
        max_point: Endpoint,
    ) {
        let (max_point_event_start_index, max_point_event_end_index) =
            self.divide(event_index, max_point);
        self.push(max_point_event_start_index);
        self.push(max_point_event_end_index);
        let (min_point_event_start_index, _) = self.divide(event_index, min_point);
        self.push(min_point_event_start_index);
    }

    fn divide_below_event_by_midpoint(&mut self, below_event_index: usize, point: Endpoint) {
        let (point_below_event_start_index, point_below_event_end_index) =
            self.divide(below_event_index, point);
        self.push(point_below_event_start_index);
        self.push(point_below_event_end_index);
    }

    fn divide_event_by_midpoint(
        &mut self,
        event_index: usize,
        point: Endpoint,
        sweep_line: &mut SweepLine<Scalar, Endpoint>,
    ) {
        if let Some(above_event_index) = sweep_line.above(event_index) {
            if self
                .get_event_start(above_event_index)
                .eq(self.get_event_start(event_index))
                && self.get_event_end(above_event_index).eq(&point)
            {
                sweep_line.remove(above_event_index);
            }
        }
        let (point_event_start_event_index, point_event_end_event_index) =
            self.divide(event_index, point);
        self.push(point_event_start_event_index);
        self.push(point_event_end_event_index);
    }
}

impl<Scalar, Endpoint: Clone + self::Point<Scalar> + Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn divide(&mut self, event_index: usize, mid_point: Endpoint) -> (usize, usize) {
        debug_assert!(event_index.is_even());
        let mid_point_to_event_end_event_index = self.endpoints().len();
        self.endpoints().push(mid_point.clone());
        self.opposites().push(self.opposites()[event_index]);
        self.opposites()[self.get_opposite_index(event_index)] = mid_point_to_event_end_event_index;
        let mid_point_to_event_start_event_index = self.endpoints().len();
        self.endpoints().push(mid_point.clone());
        self.opposites().push(event_index);
        self.opposites()[event_index] = mid_point_to_event_start_event_index;
        (
            mid_point_to_event_start_event_index,
            mid_point_to_event_end_event_index,
        )
    }
}

impl<Scalar, Endpoint: Ord> EventsQueue<Scalar, Endpoint> {
    pub(super) fn pop(&mut self) -> Option<usize> {
        self.queue.pop().map(|key| key.0.event_index)
    }

    fn push(&mut self, event_index: usize) {
        let key = Reverse(EventsQueueKey::new(
            event_index,
            self.endpoints(),
            self.opposites(),
        ));
        self.queue.push(key)
    }
}
