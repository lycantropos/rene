use core::convert::From;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::{Excluded, Unbounded};

use crate::geometries::{Point, Segment};
use crate::operations::{
    IntersectCrossingSegments, Orient, SegmentsCountable,
    ToCorrectlyOrientedSegments,
};
use crate::oriented::Orientation;
use crate::sweeping::traits::{EventsContainer, EventsQueue, SweepLine};
use crate::traits::{Elemental, Segmental};

use super::constants::UNDEFINED_INDEX;
use super::event::is_event_right;
use super::event::{
    is_event_left, left_event_to_position, segment_id_to_left_event,
    segment_id_to_right_event, Event,
};
use super::events_queue_key::EventsQueueKey;
use super::operation_kind::{DIFFERENCE, INTERSECTION};
use super::sweep_line_key::SweepLineKey;
use super::traits::ReduceEvents;

pub(crate) struct Operation<Point, const IS_FIRST_LINEAR: bool, const KIND: u8>
{
    first_segments_count: usize,
    are_from_result: Vec<bool>,
    endpoints: Box<Vec<Point>>,
    events_queue_data: BinaryHeap<Reverse<EventsQueueKey<Point>>>,
    have_interior_to_left: Vec<bool>,
    have_overlap: Vec<bool>,
    opposites: Box<Vec<Event>>,
    other_have_interior_to_left: Vec<bool>,
    segments_ids: Vec<usize>,
    starts_ids: Vec<usize>,
    sweep_line_data: BTreeSet<SweepLineKey<Point>>,
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
    > From<(&Polygon, &Segment)> for Operation<Point, false, INTERSECTION>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Polygon, &Segment)) -> Self {
        let mut result = Self::with_capacity(first.segments_count(), 1);
        result.extend(first.to_correctly_oriented_segments());
        result.extend(std::iter::once(second.clone()));
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
    > From<(&[&Polygon], &Segment)> for Operation<Point, false, INTERSECTION>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Polygon], &Segment)) -> Self {
        let mut result = Self::with_capacity(
            first
                .iter()
                .copied()
                .map(SegmentsCountable::segments_count)
                .sum::<usize>(),
            1,
        );
        for &polygon in first {
            result.extend(polygon.to_correctly_oriented_segments());
        }
        result.extend(std::iter::once(second.clone()));
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
    > From<(&Polygon, &[&Segment])> for Operation<Point, false, INTERSECTION>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Polygon, &[&Segment])) -> Self {
        let mut result =
            Self::with_capacity(first.segments_count(), second.len());
        result.extend(first.to_correctly_oriented_segments());
        result.extend(second.iter().copied().cloned());
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
    > From<(&[&Polygon], &[&Segment])>
    for Operation<Point, false, INTERSECTION>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Polygon], &[&Segment])) -> Self {
        let mut result = Self::with_capacity(
            first
                .iter()
                .copied()
                .map(SegmentsCountable::segments_count)
                .sum::<usize>(),
            second.len(),
        );
        for &polygon in first {
            result.extend(polygon.to_correctly_oriented_segments());
        }
        result.extend(second.iter().copied().cloned());
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&Segment, &Polygon)> for Operation<Point, true, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Segment, &Polygon)) -> Self {
        let mut result = Self::with_capacity(1, second.segments_count());
        result.extend(std::iter::once(first.clone()));
        result.extend(second.to_correctly_oriented_segments());
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&[&Segment], &Polygon)> for Operation<Point, true, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Segment], &Polygon)) -> Self {
        let mut result =
            Self::with_capacity(first.len(), second.segments_count());
        result.extend(first.iter().copied().cloned());
        result.extend(second.to_correctly_oriented_segments());
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&Segment, &[&Polygon])> for Operation<Point, true, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Segment, &[&Polygon])) -> Self {
        let mut result = Self::with_capacity(
            1,
            second
                .iter()
                .copied()
                .map(SegmentsCountable::segments_count)
                .sum::<usize>(),
        );
        result.extend(std::iter::once(first.clone()));
        for &polygon in second {
            result.extend(polygon.to_correctly_oriented_segments());
        }
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Clone + Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&[&Segment], &[&Polygon])> for Operation<Point, true, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Segment], &[&Polygon])) -> Self {
        let mut result = Self::with_capacity(
            first.len(),
            second
                .iter()
                .copied()
                .map(SegmentsCountable::segments_count)
                .sum::<usize>(),
        );
        result.extend(first.iter().copied().cloned());
        for &polygon in second {
            result.extend(polygon.to_correctly_oriented_segments());
        }
        result
    }
}

trait DetectIfLeftEventFromResult {
    fn detect_if_left_event_from_result(&self, event: Event) -> bool;
}

impl<Point, const IS_FIRST_LINEAR: bool> DetectIfLeftEventFromResult
    for Operation<Point, IS_FIRST_LINEAR, INTERSECTION>
{
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(event) == IS_FIRST_LINEAR
            && !self.is_left_event_outside(event)
    }
}

impl<Point> DetectIfLeftEventFromResult
    for Operation<Point, true, DIFFERENCE>
{
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(event)
            && self.is_left_event_outside(event)
    }
}

impl<
        Point: Clone + PartialOrd,
        const IS_FIRST_LINEAR: bool,
        const KIND: u8,
    > Iterator for Operation<Point, IS_FIRST_LINEAR, KIND>
where
    Self: EventsQueue<Event = Event>
        + DetectIfLeftEventFromResult
        + SweepLine<Event = Event>,
    for<'a> &'a Point:
        Elemental + IntersectCrossingSegments<Output = Point> + Orient,
    for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
{
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.pop() {
            if is_event_right(event) {
                let opposite_event = self.to_opposite_event(event);
                debug_assert!(is_event_left(opposite_event));
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
                return Some(event);
            } else if self.insert(event) {
                debug_assert!(is_event_left(event));
                let maybe_below_event = self.below(event);
                self.compute_left_event_fields(event, maybe_below_event);
                if let Some(above_event) = self.above(event) {
                    if self.detect_intersection(event, above_event) {
                        self.compute_left_event_fields(
                            event,
                            maybe_below_event,
                        );
                        self.compute_left_event_fields(
                            above_event,
                            Some(event),
                        );
                    }
                }
                if let Some(below_event) = maybe_below_event {
                    if self.detect_intersection(below_event, event) {
                        let below_below_event = self.below(below_event);
                        self.compute_left_event_fields(
                            below_event,
                            below_below_event,
                        );
                        self.compute_left_event_fields(
                            event,
                            maybe_below_event,
                        );
                    }
                }
                return Some(event);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(2 * self.events_queue_data.len()))
    }
}

impl<Scalar, const IS_FIRST_LINEAR: bool, const KIND: u8> ReduceEvents
    for Operation<Point<Scalar>, IS_FIRST_LINEAR, KIND>
where
    Segment<Scalar>: From<(Point<Scalar>, Point<Scalar>)>,
    Point<Scalar>: Clone,
{
    type Output = Vec<Segment<Scalar>>;

    fn reduce_events(&self, events: Vec<Event>) -> Self::Output {
        events
            .into_iter()
            .filter(|&event| self.is_event_from_result(event))
            .map(|event| {
                Segment::from((
                    self.get_event_start(event).clone(),
                    self.get_event_end(event).clone(),
                ))
            })
            .collect()
    }
}

impl<Point, const IS_FIRST_LINEAR: bool, const KIND: u8> EventsContainer
    for Operation<Point, IS_FIRST_LINEAR, KIND>
{
    type Endpoint = Point;
    type Event = Event;

    fn get_event_end(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[self.to_opposite_event(event)]
    }

    fn get_event_start(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[event]
    }
}

impl<Point, const IS_FIRST_LINEAR: bool, const KIND: u8>
    Operation<Point, IS_FIRST_LINEAR, KIND>
{
    pub(crate) fn to_opposite_event(&self, event: Event) -> Event {
        self.opposites[event]
    }

    fn compute_left_event_fields(
        &mut self,
        event: Event,
        maybe_below_event: Option<Event>,
    ) where
        Self: DetectIfLeftEventFromResult,
        for<'a> &'a Point: Elemental,
        for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
    {
        let event_position = left_event_to_position(event);
        if let Some(below_event) = maybe_below_event {
            let below_event_position = left_event_to_position(below_event);
            self.other_have_interior_to_left[event_position] = {
                if self.is_left_event_from_first_operand(event)
                    == self.is_left_event_from_first_operand(below_event)
                {
                    self.other_have_interior_to_left[below_event_position]
                } else {
                    self.have_interior_to_left
                        [self.left_event_to_segment_id(below_event)]
                }
            };
        }
        self.are_from_result[event_position] =
            self.detect_if_left_event_from_result(event);
    }

    fn get_endpoints(&self) -> &Vec<Point> {
        &self.endpoints
    }

    fn get_opposites(&self) -> &Vec<Event> {
        &self.opposites
    }

    fn is_event_from_first_operand(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(self.to_left_event(event))
    }

    fn is_event_from_result(&self, event: Event) -> bool {
        self.are_from_result[left_event_to_position(self.to_left_event(event))]
    }

    fn is_left_event_from_first_operand(&self, event: Event) -> bool {
        self.left_event_to_segment_id(event) < self.first_segments_count
    }

    fn is_left_event_outside(&self, event: Event) -> bool {
        let event_position = left_event_to_position(event);
        !self.other_have_interior_to_left[event_position]
            && !self.have_overlap[event_position]
    }

    fn left_event_to_segment_id(&self, event: Event) -> usize {
        self.segments_ids[left_event_to_position(event)]
    }

    fn to_events_queue_key(&self, event: Event) -> EventsQueueKey<Point> {
        EventsQueueKey::new(
            event,
            self.is_event_from_first_operand(event),
            self.get_endpoints(),
            self.get_opposites(),
        )
    }

    fn to_left_event(&self, event: Event) -> Event {
        if is_event_left(event) {
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

impl<
        Point: Clone + PartialOrd,
        const IS_FIRST_LINEAR: bool,
        const KIND: u8,
    > Operation<Point, IS_FIRST_LINEAR, KIND>
where
    Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Point: IntersectCrossingSegments<Output = Point> + Orient,
{
    fn detect_intersection(
        &mut self,
        below_event: Event,
        event: Event,
    ) -> bool {
        debug_assert_ne!(below_event, event);

        let event_start = self.get_event_start(event);
        let event_end = self.get_event_end(event);
        let below_event_start = self.get_event_start(below_event);
        let below_event_end = self.get_event_end(below_event);

        let event_start_orientation =
            below_event_end.orient(below_event_start, event_start);
        let event_end_orientation =
            below_event_end.orient(below_event_start, event_end);
        if event_start_orientation == event_end_orientation {
            if event_start_orientation == Orientation::Collinear {
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
                        let min_end =
                            self.get_event_end(min_end_event).clone();
                        let (min_end_to_start_event, min_end_to_max_end_event) =
                            self.divide(max_end_event, min_end);
                        self.push(min_end_to_start_event);
                        self.push(min_end_to_max_end_event);
                    }
                    self.have_overlap[left_event_to_position(below_event)] =
                        true;
                    self.have_overlap[left_event_to_position(event)] = true;
                    return true;
                } else if event_end == below_event_end {
                    let (max_start_event, min_start_event) =
                        if event_start < below_event_start {
                            (below_event, event)
                        } else {
                            (event, below_event)
                        };
                    let max_start =
                        self.get_event_start(max_start_event).clone();
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
        } else if event_start_orientation == Orientation::Collinear {
            if below_event_start < event_start && event_start < below_event_end
            {
                let point = event_start.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else if event_end_orientation == Orientation::Collinear {
            if below_event_start < event_end && event_end < below_event_end {
                let point = event_end.clone();
                self.divide_event_by_midpoint(below_event, point);
            }
        } else {
            let below_event_start_orientation =
                event_start.orient(event_end, below_event_start);
            let below_event_end_orientation =
                event_start.orient(event_end, below_event_end);
            if below_event_start_orientation == Orientation::Collinear {
                debug_assert_ne!(
                    below_event_end_orientation,
                    Orientation::Collinear
                );
                if event_start < below_event_start
                    && below_event_start < event_end
                {
                    let point = below_event_start.clone();
                    self.divide_event_by_midpoint(event, point);
                }
            } else if below_event_end_orientation == Orientation::Collinear {
                if event_start < below_event_end && below_event_end < event_end
                {
                    let point = below_event_end.clone();
                    self.divide_event_by_midpoint(event, point);
                }
            } else if below_event_start_orientation
                != below_event_end_orientation
            {
                let cross_point =
                    IntersectCrossingSegments::intersect_crossing_segments(
                        event_start,
                        event_end,
                        below_event_start,
                        below_event_end,
                    );
                self.divide_event_by_midpoint(
                    below_event,
                    cross_point.clone(),
                );
                self.divide_event_by_midpoint(event, cross_point);
            }
        }
        false
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

impl<Point: Clone, const IS_FIRST_LINEAR: bool, const KIND: u8>
    Operation<Point, IS_FIRST_LINEAR, KIND>
{
    fn divide(&mut self, event: Event, mid_point: Point) -> (Event, Event) {
        debug_assert!(is_event_left(event));
        let opposite_event = self.to_opposite_event(event);
        let mid_point_to_event_end_event: Event = self.endpoints.len();
        self.segments_ids.push(self.left_event_to_segment_id(event));
        self.endpoints.push(mid_point.clone());
        self.opposites.push(opposite_event);
        self.opposites[opposite_event] = mid_point_to_event_end_event;
        self.other_have_interior_to_left.push(false);
        self.are_from_result.push(false);
        self.have_overlap.push(false);
        self.starts_ids.push(UNDEFINED_INDEX);
        let mid_point_to_event_start_event: Event = self.endpoints.len();
        self.endpoints.push(mid_point);
        self.opposites.push(event);
        self.opposites[event] = mid_point_to_event_start_event;
        self.starts_ids.push(UNDEFINED_INDEX);
        debug_assert_eq!(
            self.is_left_event_from_first_operand(event),
            self.is_event_from_first_operand(mid_point_to_event_start_event)
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

impl<Point: Ord, const IS_FIRST_LINEAR: bool, const KIND: u8> EventsQueue
    for Operation<Point, IS_FIRST_LINEAR, KIND>
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

impl<Point, const IS_FIRST_LINEAR: bool, const KIND: u8> SweepLine
    for Operation<Point, IS_FIRST_LINEAR, KIND>
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

impl<Point: Ord, const IS_FIRST_LINEAR: bool, const KIND: u8>
    Operation<Point, IS_FIRST_LINEAR, KIND>
where
    for<'a> &'a Point: Orient,
{
    fn extend<Segment>(&mut self, segments: impl Iterator<Item = Segment>)
    where
        Segment: Segmental<Endpoint = Point>,
    {
        let segment_id_offset = self.endpoints.len() / 2;
        for (segment_index, segment) in segments.enumerate() {
            let (mut start, mut end) = segment.endpoints();
            debug_assert!(start != end);
            let segment_id = segment_id_offset + segment_index;
            let is_sorted_segment = start < end;
            if !is_sorted_segment {
                (start, end) = (end, start);
                self.have_interior_to_left[segment_id] = false;
            }
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
            are_from_result: vec![false; segments_count],
            endpoints: Box::new(Vec::with_capacity(initial_events_count)),
            events_queue_data: BinaryHeap::with_capacity(initial_events_count),
            have_interior_to_left: vec![true; segments_count],
            have_overlap: vec![false; segments_count],
            opposites: Box::new(Vec::with_capacity(initial_events_count)),
            other_have_interior_to_left: vec![false; segments_count],
            segments_ids: (0..segments_count).collect(),
            starts_ids: vec![UNDEFINED_INDEX; initial_events_count],
            sweep_line_data: BTreeSet::new(),
        }
    }
}
