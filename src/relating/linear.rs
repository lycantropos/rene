use core::convert::From;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::{Excluded, Unbounded};
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::operations::{
    to_sorted_pair, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::sweeping::traits::{EventsContainer, EventsQueue, SweepLine};
use crate::traits::{Elemental, Segmental};

use super::event::{
    is_event_left, is_event_right, left_event_to_position,
    segment_id_to_left_event, segment_id_to_right_event, Event,
};
use super::events_queue_key::EventsQueueKey;
use super::sweep_line_key::SweepLineKey;
use super::utils::all_equal;

pub(crate) struct Operation<Point> {
    first_segments_count: usize,
    #[allow(clippy::box_collection)]
    endpoints: Box<Vec<Point>>,
    events_queue_data: BinaryHeap<Reverse<EventsQueueKey<Point>>>,
    #[allow(clippy::box_collection)]
    opposites: Box<Vec<Event>>,
    segments_ids: Vec<usize>,
    sweep_line_data: BTreeSet<SweepLineKey<Point>>,
}

struct RelationState {
    first_is_subset: bool,
    second_is_subset: bool,
    has_crossing: bool,
    has_intersection: bool,
    has_overlap: bool,
}

impl RelationState {
    fn update<
        Output: Div<Output = Output>
            + Neg<Output = Output>
            + Ord
            + Square<Output = Output>,
        Point: PartialEq,
    >(
        &mut self,
        same_start_events: &[Event],
        operation: &Operation<Point>,
    ) where
        for<'a> &'a Output: Signed,
        for<'a> &'a Point: DotMultiply<Output = Output>
            + Orient
            + SquaredMetric<Output = Output>,
    {
        debug_assert!(!same_start_events.is_empty());
        if operation.has_intersection(same_start_events) {
            if !self.has_intersection {
                self.has_intersection = true
            }
            self.detect_touch_or_overlap(same_start_events, operation);
            self.detect_crossing(same_start_events, operation);
        } else if operation.is_event_from_first_operand(same_start_events[0]) {
            if self.first_is_subset {
                self.first_is_subset = false
            }
        } else if self.second_is_subset {
            self.second_is_subset = false
        }
    }

    fn detect_touch_or_overlap<Point: PartialEq>(
        &mut self,
        same_start_events: &[Event],
        operation: &Operation<Point>,
    ) {
        let mut left_events = same_start_events
            .iter()
            .copied()
            .filter(|&event| is_event_left(event));
        if let Some(mut prev_event) = left_events.next() {
            loop {
                if let Some(event) = left_events.next() {
                    if operation.get_event_end(event)
                        == operation.get_event_end(prev_event)
                    {
                        if !self.has_overlap {
                            self.has_overlap = true;
                        }
                        if let Some(event) = left_events.next() {
                            prev_event = event;
                        } else {
                            break;
                        }
                    } else {
                        if operation.is_event_from_first_operand(prev_event) {
                            if self.first_is_subset {
                                self.first_is_subset = false
                            }
                        } else if self.second_is_subset {
                            self.second_is_subset = false
                        }
                        prev_event = event;
                    }
                } else {
                    if operation.is_event_from_first_operand(prev_event) {
                        if self.first_is_subset {
                            self.first_is_subset = false
                        }
                    } else if self.second_is_subset {
                        self.second_is_subset = false
                    }
                    break;
                }
            }
        }
    }

    fn detect_crossing<
        Output: Div<Output = Output>
            + Neg<Output = Output>
            + Ord
            + Square<Output = Output>,
        Point,
    >(
        &mut self,
        same_start_events: &[Event],
        operation: &Operation<Point>,
    ) where
        for<'a> &'a Output: Signed,
        for<'a> &'a Point: DotMultiply<Output = Output>
            + Orient
            + SquaredMetric<Output = Output>,
    {
        if !self.has_crossing && operation.has_crossing(same_start_events) {
            self.has_crossing = true;
        }
    }
}

impl<Point: Ord, Segment: Clone + Segmental<Endpoint = Point>>
    From<(&[&Segment], &[&Segment])> for Operation<Point>
where
    for<'a> &'a Point: Orient,
{
    fn from((first, second): (&[&Segment], &[&Segment])) -> Self {
        let first_segments_count = first.len();
        let second_segments_count = second.len();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        result.extend(first.iter().copied().cloned());
        result.extend(second.iter().copied().cloned());
        result
    }
}

impl<Point: Clone + PartialOrd, Scalar> Operation<Point>
where
    Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    Scalar: PartialOrd,
{
    pub(super) fn into_relation<
        Output: Div<Output = Output>
            + Neg<Output = Output>
            + Ord
            + Square<Output = Output>,
    >(
        mut self,
        first_is_subset: bool,
        second_is_subset: bool,
        min_max_x: &Scalar,
    ) -> Relation
    where
        for<'a> &'a Output: Signed,
        for<'a> &'a Point:
            DotMultiply<Output = Output> + SquaredMetric<Output = Output>,
    {
        let mut state = RelationState {
            first_is_subset,
            second_is_subset,
            has_crossing: false,
            has_intersection: false,
            has_overlap: false,
        };
        let mut first_same_start_event =
            unsafe { self.pop().unwrap_unchecked() };
        let mut same_start_events = vec![first_same_start_event];
        self.process_event(first_same_start_event);
        loop {
            if let Some(event) = self.pop() {
                let start = self.get_event_start(event);
                if start == self.get_event_start(first_same_start_event) {
                    same_start_events.push(event);
                } else {
                    state.update(&same_start_events, &self);
                    same_start_events.clear();
                    if state.has_overlap
                        && !state.first_is_subset
                        && !state.second_is_subset
                    {
                        break;
                    }
                    if start.x().gt(min_max_x) {
                        if self.is_event_from_first_operand(event) {
                            if state.first_is_subset {
                                state.first_is_subset = false
                            }
                        } else if state.second_is_subset {
                            state.second_is_subset = false
                        }
                        break;
                    }
                    first_same_start_event = event;
                    same_start_events.push(event);
                }
                self.process_event(event);
            } else {
                debug_assert!(!same_start_events.is_empty());
                state.update(&same_start_events, &self);
                same_start_events.clear();
                break;
            }
        }
        debug_assert!(same_start_events.is_empty());
        if state.first_is_subset {
            if state.second_is_subset {
                Relation::Equal
            } else {
                Relation::Component
            }
        } else if state.second_is_subset {
            Relation::Composite
        } else if state.has_overlap {
            Relation::Overlap
        } else if state.has_crossing {
            Relation::Cross
        } else if state.has_intersection {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    }
}

impl<Point: Clone + PartialOrd> Operation<Point>
where
    Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Point:
        Elemental + IntersectCrossingSegments<Output = Point> + Orient,
    for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
{
    fn process_event(&mut self, event: Event) {
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
        } else if self.insert(event) {
            debug_assert!(is_event_left(event));
            let (maybe_above_event, maybe_below_event) =
                (self.above(event), self.below(event));
            if let Some(above_event) = maybe_above_event {
                self.detect_intersection(event, above_event);
            }
            if let Some(below_event) = maybe_below_event {
                self.detect_intersection(below_event, event);
            }
        }
    }
}

impl<Point> EventsContainer for Operation<Point> {
    type Endpoint = Point;
    type Event = Event;

    fn get_event_end(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[self.to_opposite_event(event)]
    }

    fn get_event_start(&self, event: Self::Event) -> &Self::Endpoint {
        &self.endpoints[event]
    }
}

impl<Point> Operation<Point> {
    fn has_crossing<
        Output: Div<Output = Output>
            + Neg<Output = Output>
            + Ord
            + Square<Output = Output>,
    >(
        &self,
        same_start_events: &[Event],
    ) -> bool
    where
        for<'a> &'a Output: Signed,
        for<'a> &'a Point: DotMultiply<Output = Output>
            + Orient
            + SquaredMetric<Output = Output>,
    {
        if same_start_events.len() < 4 {
            return false;
        }
        let from_first_operand_events_count = same_start_events
            .iter()
            .filter(|&&event| self.is_event_from_first_operand(event))
            .count();
        if !(1 < from_first_operand_events_count
            && from_first_operand_events_count < same_start_events.len() - 1)
        {
            // for crossing angles there should be at least two pairs of segments from each operand
            return false;
        };
        let (mut from_first_events, mut from_second_events) = (
            Vec::with_capacity(same_start_events.len()),
            Vec::with_capacity(same_start_events.len()),
        );
        for &event in same_start_events {
            (if self.is_event_from_first_operand(event) {
                &mut from_first_events
            } else {
                &mut from_second_events
            })
            .push(event);
        }
        let start = self.get_event_start(same_start_events[0]);
        let base_event = unsafe {
            from_second_events
                .iter()
                .min_by_key(|&&event| {
                    self.to_signed_point_event_squared_cosine(
                        self.get_event_end(from_second_events[0]),
                        event,
                    )
                })
                .copied()
                .unwrap_unchecked()
        };
        let base_end = self.get_event_end(base_event);
        let largest_angle_event = unsafe {
            from_second_events
                .into_iter()
                .min_by_key(|&event| {
                    self.to_signed_point_event_squared_cosine(base_end, event)
                })
                .unwrap_unchecked()
        };
        let largest_angle_end = self.get_event_end(largest_angle_event);
        let base_orientation = start.orient(base_end, largest_angle_end);
        !all_equal(from_first_events.into_iter().map(|event| {
            is_point_in_angle(
                self.get_event_end(event),
                start,
                base_end,
                largest_angle_end,
                base_orientation,
            )
        }))
    }

    fn has_intersection(&self, same_start_events: &[Event]) -> bool {
        debug_assert!(!same_start_events.is_empty());
        !all_equal(
            same_start_events
                .iter()
                .map(|&event| self.is_event_from_first_operand(event)),
        )
    }

    fn is_event_from_first_operand(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(self.to_left_event(event))
    }

    fn get_endpoints(&self) -> &Vec<Point> {
        &self.endpoints
    }

    fn get_opposites(&self) -> &Vec<Event> {
        &self.opposites
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

    fn to_opposite_event(&self, event: Event) -> Event {
        self.opposites[event]
    }

    fn to_signed_point_event_squared_cosine<
        Output: Div<Output = Output> + Neg<Output = Output> + Square<Output = Output>,
    >(
        &self,
        point: &Point,
        event: Event,
    ) -> Output
    where
        for<'a> &'a Output: Signed,
        for<'a> &'a Point:
            DotMultiply<Output = Output> + SquaredMetric<Output = Output>,
    {
        let start = self.get_event_start(event);
        let end = self.get_event_end(event);
        let dot_product = DotMultiply::dot_multiply(start, point, start, end);
        (if dot_product.is_positive() {
            dot_product.square()
        } else {
            -dot_product.square()
        }) / start.squared_distance_to(end)
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

impl<Point: Clone + PartialOrd> Operation<Point>
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

impl<Point: Clone> Operation<Point> {
    fn divide(&mut self, event: Event, mid_point: Point) -> (Event, Event) {
        debug_assert!(is_event_left(event));
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

impl<Point: Ord> EventsQueue for Operation<Point>
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

impl<Point> SweepLine for Operation<Point>
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

impl<Point: Ord> Operation<Point>
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

fn is_point_in_angle<Point>(
    point: &Point,
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
    angle_orientation: Orientation,
) -> bool
where
    for<'a> &'a Point: Orient,
{
    let first_half_orientation = vertex.orient(first_ray_point, point);
    let second_half_orientation = vertex.orient(point, second_ray_point);
    if first_half_orientation == Orientation::Collinear {
        second_half_orientation == angle_orientation
    } else if second_half_orientation == Orientation::Collinear {
        first_half_orientation == angle_orientation
    } else {
        (first_half_orientation == second_half_orientation)
            && first_half_orientation
                == (if angle_orientation == Orientation::Collinear {
                    Orientation::Counterclockwise
                } else {
                    angle_orientation
                })
    }
}
