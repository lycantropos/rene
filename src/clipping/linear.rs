use core::convert::From;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::{Excluded, Unbounded};

use crate::geometries::{Point, Segment};
use crate::operations::{to_sorted_pair, IntersectCrossingSegments, Orient};
use crate::oriented::Orientation;
use crate::sweeping::traits::{EventsContainer, EventsQueue, SweepLine};
use crate::traits::{Elemental, Segmental};

use super::event::is_right_event;
use super::event::{
    is_left_event, left_event_to_position, segment_id_to_left_event,
    segment_id_to_right_event, Event,
};
use super::events_queue_key::EventsQueueKey;
use super::operation_kind::{
    DIFFERENCE, INTERSECTION, SYMMETRIC_DIFFERENCE, UNION,
};
use super::sweep_line_key::SweepLineKey;
use super::traits::ReduceEvents;

pub(crate) struct Operation<Point, const KIND: u8> {
    first_segments_count: usize,
    endpoints: Box<Vec<Point>>,
    events_queue_data: BinaryHeap<Reverse<EventsQueueKey<Point>>>,
    opposites: Box<Vec<Event>>,
    segments_ids: Vec<usize>,
    sweep_line_data: BTreeSet<SweepLineKey<Point>>,
}

impl<First: Clone, Point: Ord, Second: Clone, const KIND: u8>
    From<(&[&First], &[&Second])> for Operation<Point, KIND>
where
    First: Segmental<Endpoint = Point>,
    Second: Segmental<Endpoint = Point>,
    for<'a> &'a Point: Orient,
{
    fn from((first, second): (&[&First], &[&Second])) -> Self {
        let first_segments_count = first.len();
        let second_segments_count = second.len();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        result.extend(first.iter().copied().cloned());
        result.extend(second.iter().copied().cloned());
        result
    }
}

impl<Point: Clone + PartialOrd, const KIND: u8> Iterator
    for Operation<Point, KIND>
where
    Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Point:
        Elemental + IntersectCrossingSegments<Output = Point> + Orient,
    for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.pop() {
            if is_right_event(event) {
                let opposite_event = self.to_opposite_event(event);
                debug_assert!(is_left_event(opposite_event));
                if let Some(equal_segment_event) =
                    <Self as SweepLine>::find(self, opposite_event)
                {
                    let (maybe_above_event, maybe_below_event) = (
                        self.above(equal_segment_event),
                        self.below(equal_segment_event),
                    );
                    self.remove(equal_segment_event);
                    if let (Some(above_event), Some(below_event)) =
                        (maybe_above_event, maybe_below_event)
                    {
                        self.detect_intersection(below_event, above_event);
                    }
                }
            } else if self.insert(event) {
                debug_assert!(is_left_event(event));
                let (maybe_above_event, maybe_below_event) =
                    (self.above(event), self.below(event));
                if let Some(above_event) = maybe_above_event {
                    self.detect_intersection(event, above_event);
                }
                if let Some(below_event) = maybe_below_event {
                    self.detect_intersection(below_event, event);
                }
            }
            Some(event)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(2 * self.events_queue_data.len()))
    }
}

impl<Scalar> ReduceEvents for Operation<Point<Scalar>, INTERSECTION>
where
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    EventsQueueKey<Point<Scalar>>: Ord,
    Segment<Scalar>: From<(Point<Scalar>, Point<Scalar>)>,
    Point<Scalar>: Clone + PartialEq,
    for<'a> &'a Point<Scalar>: Elemental + Orient,
{
    type Output = Vec<Segment<Scalar>>;

    fn reduce_events(&self, events: Vec<Event>) -> Self::Output {
        if events.is_empty() {
            vec![]
        } else {
            let mut result = Vec::with_capacity(events.len() / 2);
            let mut events = events.into_iter();
            let event = unsafe { events.next().unwrap_unchecked() };
            let (mut previous_end, mut previous_start) =
                (self.get_event_end(event), self.get_event_start(event));
            for event in events {
                let (end, start) =
                    (self.get_event_end(event), self.get_event_start(event));
                if end == previous_end && start == previous_start {
                    result.push(Segment::from((start.clone(), end.clone())));
                } else {
                    (previous_end, previous_start) = (end, start);
                }
            }
            result
        }
    }
}

impl<Point, const KIND: u8> EventsContainer for Operation<Point, KIND> {
    type Endpoint = Point;
    type Event = Event;

    fn get_event_end(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[self.to_opposite_event(event)]
    }

    fn get_event_start(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[event]
    }
}

impl<Point, const KIND: u8> Operation<Point, KIND> {
    pub(crate) fn to_opposite_event(&self, event: Event) -> Event {
        self.opposites[event]
    }

    fn get_endpoints(&self) -> &Vec<Point> {
        &self.endpoints
    }

    fn get_opposites(&self) -> &Vec<Event> {
        &self.opposites
    }

    fn is_from_first_operand_event(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(self.to_left_event(event))
    }

    fn is_left_event_from_first_operand(&self, event: Event) -> bool {
        self.left_event_to_segment_id(event) < self.first_segments_count
    }

    fn left_event_to_segment_id(&self, event: Event) -> usize {
        self.segments_ids[left_event_to_position(event)]
    }

    fn to_events_queue_key(&self, event: Event) -> EventsQueueKey<Point> {
        EventsQueueKey::new(
            event,
            self.is_from_first_operand_event(event),
            self.get_endpoints(),
            self.get_opposites(),
        )
    }

    fn to_left_event(&self, event: Event) -> Event {
        if is_left_event(event) {
            event
        } else {
            self.to_opposite_event(event)
        }
    }

    fn to_sweep_line_key(&self, event: Event) -> SweepLineKey<Point> {
        SweepLineKey::new(
            event,
            self.is_left_event_from_first_operand(event),
            &self.endpoints,
            &self.opposites,
        )
    }
}

impl<Point: Clone + PartialOrd, const KIND: u8> Operation<Point, KIND>
where
    Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Point: IntersectCrossingSegments<Output = Point> + Orient,
{
    fn detect_intersection(&mut self, below_event: Event, event: Event) {
        debug_assert_ne!(below_event, event);

        let event_start = self.get_event_start(event);
        let event_end = self.get_event_end(event);
        let below_event_start = self.get_event_start(below_event);
        let below_event_end = self.get_event_end(below_event);

        let event_start_orientation =
            below_event_end.orient(below_event_start, event_start);
        let event_end_orientation =
            below_event_end.orient(below_event_start, event_end);
        if event_start_orientation != Orientation::Collinear
            && event_end_orientation != Orientation::Collinear
        {
            if event_start_orientation != event_end_orientation {
                let below_event_start_orientation =
                    event_start.orient(event_end, below_event_start);
                let below_event_end_orientation =
                    event_start.orient(event_end, below_event_end);
                if below_event_start_orientation != Orientation::Collinear
                    && below_event_end_orientation != Orientation::Collinear
                {
                    if below_event_start_orientation
                        != below_event_end_orientation
                    {
                        let point =
                            IntersectCrossingSegments::intersect_crossing_segments(
                                event_start,
                                event_end,
                                below_event_start,
                                below_event_end,
                            );
                        self.divide_event_by_midpoint(
                            below_event,
                            point.clone(),
                        );
                        self.divide_event_by_midpoint(event, point);
                    }
                } else if below_event_start_orientation
                    != Orientation::Collinear
                {
                    if event_start < below_event_end
                        && below_event_end < event_end
                    {
                        let point = below_event_end.clone();
                        self.divide_event_by_midpoint(event, point);
                    }
                } else if event_start < below_event_start
                    && below_event_start < event_end
                {
                    let point = below_event_start.clone();
                    self.divide_event_by_midpoint(event, point);
                }
            }
        } else if event_end_orientation != Orientation::Collinear {
            if below_event_start < event_start && event_start < below_event_end
            {
                let point = event_start.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else if event_start_orientation != Orientation::Collinear {
            if below_event_start < event_end && event_end < below_event_end {
                let point = event_end.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else {
            // overlap
            debug_assert_ne!(
                self.is_left_event_from_first_operand(below_event),
                self.is_left_event_from_first_operand(event)
            );

            if event_start == below_event_start {
                if event_end != below_event_end {
                    let (max_end_event, min_end_event) =
                        if event_end < below_event_end {
                            (below_event, event)
                        } else {
                            (event, below_event)
                        };
                    let min_end = self.get_event_end(min_end_event).clone();
                    let (min_end_to_start_event, min_end_to_max_end_event) =
                        self.divide(max_end_event, min_end);
                    self.push(min_end_to_start_event);
                    self.push(min_end_to_max_end_event);
                }
            } else if event_end == below_event_end {
                let (max_start_event, min_start_event) =
                    if event_start < below_event_start {
                        (below_event, event)
                    } else {
                        (event, below_event)
                    };
                let max_start = self.get_event_start(max_start_event).clone();
                let (max_start_to_min_start_event, max_start_to_end_event) =
                    self.divide(min_start_event, max_start);
                self.push(max_start_to_min_start_event);
                self.push(max_start_to_end_event);
            } else if below_event_start < event_start
                && event_start < below_event_end
            {
                if event_end < below_event_end {
                    let event_start = event_start.clone();
                    let event_end = event_end.clone();
                    self.divide_event_by_mid_segment_event_endpoints(
                        below_event,
                        event,
                        event_start,
                        event_end,
                    );
                } else {
                    let (max_start, min_end) =
                        (event_start.clone(), below_event_end.clone());
                    self.divide_overlapping_events(
                        below_event,
                        event,
                        max_start,
                        min_end,
                    );
                }
            } else if event_start < below_event_start
                && below_event_start < event_end
            {
                if below_event_end < event_end {
                    let below_event_start = below_event_start.clone();
                    let below_event_end = below_event_end.clone();
                    self.divide_event_by_mid_segment_event_endpoints(
                        event,
                        below_event,
                        below_event_start,
                        below_event_end,
                    );
                } else {
                    let (max_start, min_end) =
                        (below_event_start.clone(), event_end.clone());
                    self.divide_overlapping_events(
                        event,
                        below_event,
                        max_start,
                        min_end,
                    );
                }
            }
        }
    }

    fn divide_overlapping_events(
        &mut self,
        min_start_event: Event,
        max_start_event: Event,
        max_start: Point,
        min_end: Point,
    ) {
        self.divide_event_by_midpoint(max_start_event, min_end);
        self.divide_event_by_midpoint(min_start_event, max_start);
    }

    fn divide_event_by_mid_segment_event_endpoints(
        &mut self,
        event: Event,
        mid_segment_event: Event,
        mid_segment_event_start: Point,
        mid_segment_event_end: Point,
    ) where
        Point: PartialEq,
    {
        debug_assert!(mid_segment_event_start
            .eq(self.get_event_start(mid_segment_event)));
        debug_assert!(
            mid_segment_event_end.eq(self.get_event_end(mid_segment_event))
        );
        debug_assert!(mid_segment_event_start.ne(self.get_event_start(event)));
        debug_assert!(mid_segment_event_end.ne(self.get_event_end(event)));

        self.divide_event_by_midpoint(event, mid_segment_event_end);
        self.divide_event_by_midpoint(event, mid_segment_event_start);
    }

    fn divide_event_by_midpoint(&mut self, event: Event, point: Point) {
        let (point_to_event_start_event, point_to_event_end_event) =
            self.divide(event, point);
        self.push(point_to_event_start_event);
        self.push(point_to_event_end_event);
    }
}

impl<Point: Clone, const KIND: u8> Operation<Point, KIND> {
    fn divide(&mut self, event: Event, mid_point: Point) -> (Event, Event) {
        debug_assert!(is_left_event(event));
        let opposite_event = self.to_opposite_event(event);
        let mid_point_to_event_end_event: Event = self.endpoints.len();
        self.segments_ids.push(self.left_event_to_segment_id(event));
        self.endpoints.push(mid_point.clone());
        self.opposites.push(opposite_event);
        self.opposites[opposite_event] = mid_point_to_event_end_event;
        let mid_point_to_event_start_event: Event = self.endpoints.len();
        self.endpoints.push(mid_point);
        self.opposites.push(event);
        self.opposites[event] = mid_point_to_event_start_event;
        debug_assert_eq!(
            self.is_left_event_from_first_operand(event),
            self.is_from_first_operand_event(mid_point_to_event_start_event)
        );
        debug_assert_eq!(
            self.is_left_event_from_first_operand(event),
            self.is_left_event_from_first_operand(
                mid_point_to_event_end_event
            )
        );
        (mid_point_to_event_start_event, mid_point_to_event_end_event)
    }
}

impl<Point: Ord, const KIND: u8> EventsQueue for Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
{
    type Event = Event;

    fn peek(&mut self) -> Option<Self::Event> {
        self.events_queue_data.peek().map(|key| key.0.event)
    }

    fn pop(&mut self) -> Option<Self::Event> {
        self.events_queue_data.pop().map(|key| key.0.event)
    }

    fn push(&mut self, event: Self::Event) {
        self.events_queue_data
            .push(Reverse(self.to_events_queue_key(event)));
    }
}

impl<Point, const KIND: u8> SweepLine for Operation<Point, KIND>
where
    SweepLineKey<Point>: Ord,
{
    type Event = Event;

    fn above(&self, event: Self::Event) -> Option<Self::Event> {
        self.sweep_line_data
            .range((Excluded(&self.to_sweep_line_key(event)), Unbounded))
            .next()
            .map(|key| key.event)
    }

    fn below(&self, event: Self::Event) -> Option<Self::Event> {
        self.sweep_line_data
            .range((Unbounded, Excluded(&self.to_sweep_line_key(event))))
            .last()
            .map(|key| key.event)
    }

    fn find(&self, event: Self::Event) -> Option<Self::Event> {
        self.sweep_line_data
            .get(&self.to_sweep_line_key(event))
            .map(|key| key.event)
    }

    fn insert(&mut self, event: Self::Event) -> bool {
        self.sweep_line_data.insert(self.to_sweep_line_key(event))
    }

    fn remove(&mut self, event: Self::Event) -> bool {
        self.sweep_line_data.remove(&self.to_sweep_line_key(event))
    }
}

impl<Point: Ord, const KIND: u8> Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
{
    fn extend<Segment>(&mut self, segments: impl Iterator<Item = Segment>)
    where
        Segment: Segmental<Endpoint = Point>,
    {
        let segment_id_offset = self.endpoints.len() / 2;
        for (segment_index, segment) in segments.enumerate() {
            let (start, end) = to_sorted_pair(segment.endpoints());
            debug_assert!(start != end);
            let segment_id = segment_id_offset + segment_index;
            let left_event = segment_id_to_left_event(segment_id);
            let right_event = segment_id_to_right_event(segment_id);
            self.endpoints.push(start);
            self.endpoints.push(end);
            self.opposites.push(right_event);
            self.opposites.push(left_event);
            self.push(left_event);
            self.push(right_event);
        }
    }

    fn with_capacity(
        first_segments_count: usize,
        second_segments_count: usize,
    ) -> Self {
        let segments_count = first_segments_count + second_segments_count;
        let initial_events_count = 2 * segments_count;
        Self {
            first_segments_count,
            endpoints: Box::new(Vec::with_capacity(initial_events_count)),
            events_queue_data: BinaryHeap::with_capacity(initial_events_count),
            opposites: Box::new(Vec::with_capacity(initial_events_count)),
            segments_ids: (0..segments_count).collect(),
            sweep_line_data: BTreeSet::new(),
        }
    }
}
