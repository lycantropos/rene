use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::{Excluded, Unbounded};

use crate::operations::{IntersectCrossingSegments, Orient};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::relating::event::is_event_right;
use crate::relating::utils::all_equal;
use crate::sweeping::traits::{EventsContainer, EventsQueue, SweepLine};
use crate::traits::{Elemental, Segmental};

use super::event::{
    is_event_left, left_event_to_position, segment_id_to_left_event,
    segment_id_to_right_event, Event,
};
use super::events_queue_key::EventsQueueKey;
use super::sweep_line_key::SweepLineKey;

pub(crate) struct Operation<Point> {
    first_segments_count: usize,
    endpoints: Box<Vec<Point>>,
    events_queue_data: BinaryHeap<Reverse<EventsQueueKey<Point>>>,
    have_interior_to_left: Vec<bool>,
    opposites: Box<Vec<Event>>,
    other_have_interior_to_left: Vec<bool>,
    overlap_kinds: Vec<OverlapKind>,
    segments_ids: Vec<usize>,
    sweep_line_data: BTreeSet<SweepLineKey<Point>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EventKind {
    CommonPolylineSegment,
    CommonRegionEdge,
    Inside,
    Outside,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OverlapKind {
    None,
    SameOrientation,
    DifferentOrientation,
}

struct RelationState {
    boundaries_intersect: bool,
    first_boundary_intersects_second_interior: bool,
    first_is_subset: bool,
    has_continuous_intersection: bool,
    second_boundary_intersects_first_interior: bool,
    second_is_subset: bool,
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

impl<Point: Ord> Operation<Point>
where
    for<'a> &'a Point: Orient,
{
    pub(super) fn from_segments_iterators<
        First: Iterator<Item = Segment>,
        Second: Iterator<Item = Segment>,
        Segment: Segmental<Endpoint = Point>,
    >(
        (first_segments_count, first_segments): (usize, First),
        (second_segments_count, second_segments): (usize, Second),
    ) -> Self {
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        result.extend(first_segments);
        result.extend(second_segments);
        result
    }
}

impl<Point> Operation<Point> {
    pub(super) fn into_relation<Scalar: PartialOrd>(
        mut self,
        first_is_subset: bool,
        second_is_subset: bool,
        min_max_x: &Scalar,
    ) -> Relation
    where
        Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
        Point: Clone + PartialOrd,
        for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
            + IntersectCrossingSegments<Output = Point>
            + Orient,
    {
        let mut state = RelationState {
            boundaries_intersect: false,
            first_boundary_intersects_second_interior: false,
            first_is_subset,
            second_boundary_intersects_first_interior: false,
            second_is_subset,
            has_continuous_intersection: false,
        };
        let event = unsafe { self.pop().unwrap_unchecked() };
        let mut first_same_start_event = event;
        let mut same_start_events = vec![event];
        self.process_event(event);
        loop {
            if let Some(event) = self.pop() {
                let start = self.get_event_start(event);
                if start == self.get_event_start(first_same_start_event) {
                    same_start_events.push(event);
                } else {
                    state.update(&same_start_events, &self);
                    same_start_events.clear();
                    if state.has_continuous_intersection
                        && !state.first_is_subset
                        && !state.second_is_subset
                    {
                        break;
                    }
                    if start.x() > min_max_x {
                        if self.is_event_from_first_operand(event) {
                            if state.first_is_subset {
                                state.first_is_subset = false;
                            }
                        } else if state.second_is_subset {
                            state.second_is_subset = false;
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
        if state.boundaries_intersect {
            if state.first_is_subset {
                if state.second_is_subset {
                    Relation::Equal
                } else if state.first_boundary_intersects_second_interior {
                    Relation::Enclosed
                } else {
                    Relation::Component
                }
            } else if state.second_is_subset {
                if state.second_boundary_intersects_first_interior {
                    Relation::Encloses
                } else {
                    Relation::Composite
                }
            } else {
                if state.has_continuous_intersection {
                    Relation::Overlap
                } else {
                    Relation::Touch
                }
            }
        } else {
            if state.second_is_subset {
                Relation::Cover
            } else if state.first_is_subset {
                Relation::Within
            } else if state.has_continuous_intersection {
                Relation::Overlap
            } else {
                Relation::Disjoint
            }
        }
    }
}

impl<Point> Operation<Point> {
    fn compute_left_event_fields(
        &mut self,
        event: Event,
        maybe_below_event: Option<Event>,
    ) where
        for<'a> &'a Point: Elemental,
        for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
    {
        if let Some(below_event) = maybe_below_event {
            self.other_have_interior_to_left[left_event_to_position(event)] = {
                if self.is_left_event_from_first_operand(event)
                    == self.is_left_event_from_first_operand(below_event)
                {
                    self.other_have_interior_to_left
                        [left_event_to_position(below_event)]
                } else {
                    self.have_interior_to_left
                        [self.left_event_to_segment_id(below_event)]
                }
            };
        }
    }

    fn get_endpoints(&self) -> &Vec<Point> {
        &self.endpoints
    }

    fn get_opposites(&self) -> &Vec<Event> {
        &self.opposites
    }

    fn classify_event(&self, event: Event) -> EventKind {
        let left_event_position =
            left_event_to_position(self.to_left_event(event));
        match self.overlap_kinds[left_event_position] {
            OverlapKind::None => {
                if self.other_have_interior_to_left[left_event_position] {
                    EventKind::Inside
                } else {
                    EventKind::Outside
                }
            }
            OverlapKind::DifferentOrientation => {
                EventKind::CommonPolylineSegment
            }
            OverlapKind::SameOrientation => EventKind::CommonRegionEdge,
        }
    }

    fn has_edges_cross(&self, same_start_events: &[Event]) -> bool {
        let mut flags = [None, None];
        for &event in same_start_events {
            let left_event = self.to_left_event(event);
            let left_event_position = left_event_to_position(left_event);
            if self.overlap_kinds[left_event_position] == OverlapKind::None {
                let flag_index =
                    if self.is_left_event_from_first_operand(left_event) {
                        1usize
                    } else {
                        0usize
                    };
                let event_is_inside =
                    self.other_have_interior_to_left[left_event_position];
                if let Some(previous_event_is_inside) = flags[flag_index] {
                    if previous_event_is_inside != event_is_inside {
                        return true;
                    }
                } else {
                    flags[flag_index] = Some(event_is_inside);
                }
            }
        }
        false
    }

    fn is_event_from_first_operand(&self, event: Event) -> bool {
        self.is_left_event_from_first_operand(self.to_left_event(event))
    }

    fn is_left_event_from_first_operand(&self, event: Event) -> bool {
        self.left_event_to_segment_id(event) < self.first_segments_count
    }

    fn left_event_to_segment_id(&self, event: Event) -> usize {
        self.segments_ids[left_event_to_position(event)]
    }

    fn process_event(&mut self, event: Event)
    where
        Self: EventsQueue<Event = Event> + SweepLine<Event = Event>,
        Point: Clone + PartialOrd,
        for<'a> &'a Point:
            Elemental + IntersectCrossingSegments<Output = Point> + Orient,
        for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
    {
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
            let maybe_below_event = self.below(event);
            self.compute_left_event_fields(event, maybe_below_event);
            if let Some(above_event) = self.above(event) {
                if self.detect_intersection(event, above_event) {
                    self.compute_left_event_fields(event, maybe_below_event);
                    self.compute_left_event_fields(above_event, Some(event));
                }
            }
            if let Some(below_event) = maybe_below_event {
                if self.detect_intersection(below_event, event) {
                    let below_below_event = self.below(below_event);
                    self.compute_left_event_fields(
                        below_event,
                        below_below_event,
                    );
                    self.compute_left_event_fields(event, maybe_below_event);
                }
            }
        }
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
                    let overlap_kind = if self.have_interior_to_left
                        [self.left_event_to_segment_id(event)]
                        == self.have_interior_to_left
                            [self.left_event_to_segment_id(below_event)]
                    {
                        OverlapKind::SameOrientation
                    } else {
                        OverlapKind::DifferentOrientation
                    };
                    self.overlap_kinds[left_event_to_position(below_event)] =
                        overlap_kind;
                    self.overlap_kinds[left_event_to_position(event)] =
                        overlap_kind;
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

impl<Point: Clone> Operation<Point> {
    fn divide(&mut self, event: Event, mid_point: Point) -> (Event, Event) {
        debug_assert!(is_event_left(event));
        let opposite_event = self.to_opposite_event(event);
        let mid_point_to_event_end_event: Event = self.endpoints.len();
        self.segments_ids.push(self.left_event_to_segment_id(event));
        self.endpoints.push(mid_point.clone());
        self.opposites.push(opposite_event);
        self.opposites[opposite_event] = mid_point_to_event_end_event;
        self.other_have_interior_to_left.push(false);
        self.overlap_kinds.push(OverlapKind::None);
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
            endpoints: Box::new(Vec::with_capacity(initial_events_count)),
            events_queue_data: BinaryHeap::with_capacity(initial_events_count),
            have_interior_to_left: vec![true; segments_count],
            opposites: Box::new(Vec::with_capacity(initial_events_count)),
            other_have_interior_to_left: vec![false; segments_count],
            overlap_kinds: vec![OverlapKind::None; segments_count],
            segments_ids: (0..segments_count).collect(),
            sweep_line_data: BTreeSet::new(),
        }
    }
}

impl RelationState {
    fn update<Scalar>(
        &mut self,
        same_start_events: &[Event],
        operation: &Operation<Scalar>,
    ) {
        if all_equal(
            same_start_events
                .iter()
                .map(|&event| operation.is_event_from_first_operand(event)),
        ) {
            let event = same_start_events[0];
            match operation.classify_event(event) {
                EventKind::Inside => {
                    if !self.has_continuous_intersection {
                        self.has_continuous_intersection = true;
                    }
                    if operation.is_event_from_first_operand(event) {
                        if !self.first_boundary_intersects_second_interior {
                            self.first_boundary_intersects_second_interior =
                                true;
                        }
                        if self.second_is_subset {
                            self.second_is_subset = false;
                        }
                    } else {
                        if !self.second_boundary_intersects_first_interior {
                            self.second_boundary_intersects_first_interior =
                                true;
                        }
                        if self.first_is_subset {
                            self.first_is_subset = false;
                        }
                    }
                }
                EventKind::Outside => {
                    if operation.is_event_from_first_operand(event) {
                        if self.first_is_subset {
                            self.first_is_subset = false;
                        }
                    } else if self.second_is_subset {
                        self.second_is_subset = false;
                    }
                }
                _ => unreachable!(),
            }
        } else if operation.has_edges_cross(same_start_events) {
            self.has_continuous_intersection = true;
            self.first_is_subset = false;
            self.second_is_subset = false;
        } else {
            if !self.boundaries_intersect {
                self.boundaries_intersect = true;
            }
            for &event in same_start_events {
                match operation.classify_event(event) {
                    EventKind::CommonRegionEdge => {
                        if !self.has_continuous_intersection {
                            self.has_continuous_intersection = true;
                        }
                    }
                    EventKind::Inside => {
                        if !self.has_continuous_intersection {
                            self.has_continuous_intersection = true;
                        }
                        if operation.is_event_from_first_operand(event) {
                            if !self.first_boundary_intersects_second_interior
                            {
                                self.first_boundary_intersects_second_interior = true;
                            }
                            if self.second_is_subset {
                                self.second_is_subset = false;
                            }
                        } else {
                            if !self.second_boundary_intersects_first_interior
                            {
                                self.second_boundary_intersects_first_interior = true;
                            }
                            if self.first_is_subset {
                                self.first_is_subset = false;
                            }
                        }
                    }
                    EventKind::Outside | EventKind::CommonPolylineSegment => {
                        if operation.is_event_from_first_operand(event) {
                            if self.first_is_subset {
                                self.first_is_subset = false;
                            }
                        } else if self.second_is_subset {
                            self.second_is_subset = false;
                        }
                    }
                }
            }
        }
    }
}
