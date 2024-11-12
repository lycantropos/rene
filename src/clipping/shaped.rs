use core::convert::From;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
use std::ops::Bound::{Excluded, Unbounded};

use traiter::numbers::Parity;

use crate::geometries::{Contour, Point, Polygon};
use crate::operations::{
    shrink_collinear_vertices, IntersectCrossingSegments, Orient,
    SegmentsCountable, ToCorrectlyOrientedSegments,
};
use crate::oriented::Orientation;
use crate::sweeping::traits::{EventsContainer, EventsQueue, SweepLine};
use crate::traits::{Elemental, Segmental};

use super::constants::UNDEFINED_INDEX;
use super::event::is_event_right;
use super::event::{
    is_event_left, left_event_to_position, segment_id_to_left_event,
    segment_id_to_right_event, Event, UNDEFINED_EVENT,
};
use super::events_queue_key::EventsQueueKey;
use super::operation_kind::{
    DIFFERENCE, INTERSECTION, SYMMETRIC_DIFFERENCE, UNION,
};
use super::sweep_line_key::SweepLineKey;
use super::traits::ReduceEvents;

pub(crate) struct Operation<Point, const KIND: u8> {
    first_segments_count: usize,
    are_from_result: Vec<bool>,
    below_event_from_result: Vec<Event>,
    current_endpoint_first_event: Event,
    current_endpoint_id: usize,
    #[allow(clippy::box_collection)]
    endpoints: Box<Vec<Point>>,
    events_queue_data: BinaryHeap<Reverse<EventsQueueKey<Point>>>,
    have_interior_to_left: Vec<bool>,
    #[allow(clippy::box_collection)]
    opposites: Box<Vec<Event>>,
    other_have_interior_to_left: Vec<bool>,
    overlap_kinds: Vec<OverlapKind>,
    segments_ids: Vec<usize>,
    starts_ids: Vec<usize>,
    sweep_line_data: BTreeSet<SweepLineKey<Point>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OverlapKind {
    None,
    SameOrientation,
    DifferentOrientation,
}

impl<
        Point: Ord,
        Polygon,
        Segment: Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&Polygon, &Polygon)> for Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Polygon, &Polygon)) -> Self {
        let first_segments_count = first.segments_count();
        let second_segments_count = second.segments_count();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        result.extend(first.to_correctly_oriented_segments());
        result.extend(second.to_correctly_oriented_segments());
        let first_event = unsafe { result.peek().unwrap_unchecked() };
        result.current_endpoint_first_event = first_event;
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&[&Polygon], &[&Polygon])> for Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Polygon], &[&Polygon])) -> Self {
        let first_segments_count = first
            .iter()
            .copied()
            .map(SegmentsCountable::segments_count)
            .sum::<usize>();
        let second_segments_count = second
            .iter()
            .copied()
            .map(SegmentsCountable::segments_count)
            .sum::<usize>();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        for &first_subpolygon in first {
            result.extend(first_subpolygon.to_correctly_oriented_segments());
        }
        for &second_subpolygon in second {
            result.extend(second_subpolygon.to_correctly_oriented_segments());
        }
        let first_event = unsafe { result.peek().unwrap_unchecked() };
        result.current_endpoint_first_event = first_event;
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&[&Polygon], &Polygon)> for Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&[&Polygon], &Polygon)) -> Self {
        let first_segments_count = first
            .iter()
            .copied()
            .map(SegmentsCountable::segments_count)
            .sum::<usize>();
        let second_segments_count = second.segments_count();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        for first_subpolygon in first {
            result.extend(first_subpolygon.to_correctly_oriented_segments());
        }
        result.extend(second.to_correctly_oriented_segments());
        let first_event = unsafe { result.peek().unwrap_unchecked() };
        result.current_endpoint_first_event = first_event;
        result
    }
}

impl<
        Point: Ord,
        Polygon,
        Segment: Segmental<Endpoint = Point>,
        Segments: Iterator<Item = Segment>,
        const KIND: u8,
    > From<(&Polygon, &[&Polygon])> for Operation<Point, KIND>
where
    for<'a> &'a Point: Orient,
    for<'a> &'a Polygon:
        SegmentsCountable + ToCorrectlyOrientedSegments<Output = Segments>,
{
    fn from((first, second): (&Polygon, &[&Polygon])) -> Self {
        let first_segments_count = first.segments_count();
        let second_segments_count = second
            .iter()
            .copied()
            .map(SegmentsCountable::segments_count)
            .sum::<usize>();
        let mut result =
            Self::with_capacity(first_segments_count, second_segments_count);
        result.extend(first.to_correctly_oriented_segments());
        for second_subpolygon in second {
            result.extend(second_subpolygon.to_correctly_oriented_segments());
        }
        let first_event = unsafe { result.peek().unwrap_unchecked() };
        result.current_endpoint_first_event = first_event;
        result
    }
}

trait DetectIfLeftEventFromResult {
    fn detect_if_left_event_from_result(&self, event: Event) -> bool;
}

impl<Point> DetectIfLeftEventFromResult for Operation<Point, DIFFERENCE> {
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        if self.is_left_event_from_first_operand(event) {
            self.is_left_event_outside(event)
        } else {
            self.is_left_event_inside(event)
                || self.is_left_event_common_polyline_component(event)
        }
    }
}

impl<Point> DetectIfLeftEventFromResult for Operation<Point, INTERSECTION> {
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        self.is_left_event_inside(event)
            || !self.is_left_event_from_first_operand(event)
                && self.is_left_event_common_region_boundary(event)
    }
}

impl<Point> DetectIfLeftEventFromResult
    for Operation<Point, SYMMETRIC_DIFFERENCE>
{
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        !self.is_left_event_overlapping(event)
    }
}

impl<Point> DetectIfLeftEventFromResult for Operation<Point, UNION> {
    fn detect_if_left_event_from_result(&self, event: Event) -> bool {
        self.is_left_event_outside(event)
            || (!self.is_left_event_from_first_operand(event)
                && self.is_left_event_common_region_boundary(event))
    }
}

impl<Point: Clone + PartialOrd, const KIND: u8> Iterator
    for Operation<Point, KIND>
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
            if self.get_event_start(self.current_endpoint_first_event)
                != self.get_event_start(event)
            {
                self.current_endpoint_first_event = event;
                self.current_endpoint_id += 1;
            }
            self.starts_ids[event] = self.current_endpoint_id;
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

impl<Scalar, const KIND: u8> ReduceEvents for Operation<Point<Scalar>, KIND>
where
    Contour<Scalar>: From<Vec<Point<Scalar>>>,
    EventsQueueKey<Point<Scalar>>: Ord,
    Polygon<Scalar>: From<(Contour<Scalar>, Vec<Contour<Scalar>>)>,
    Point<Scalar>: Clone + PartialEq,
    for<'a> &'a Point<Scalar>: Elemental + Orient,
{
    type Output = Vec<Polygon<Scalar>>;

    fn reduce_events(&self, events: Vec<Event>) -> Self::Output {
        let mut events = events
            .into_iter()
            .filter(|&event| self.is_event_from_result(event))
            .collect::<Vec<Event>>();
        if events.is_empty() {
            return vec![];
        }
        events.sort_by_cached_key(|&event| self.to_events_queue_key(event));
        let mut events_ids = vec![UNDEFINED_INDEX; self.events_count()];
        for (event_id, &event) in events.iter().enumerate() {
            events_ids[event] = event_id;
        }
        debug_assert!(events
            .iter()
            .all(|&event| events_ids[self.to_opposite_event(event)]
                != UNDEFINED_INDEX));
        debug_assert!(events.iter().all(|&event| self.to_start_id(event)
            < self.to_unique_visited_endpoints_count()));
        let connectivity = self.events_to_connectivity(&events);
        let mut are_internal = Vec::<bool>::new();
        let mut depths = Vec::<usize>::new();
        let mut holes = Vec::<Vec<usize>>::new();
        let mut parents = Vec::<usize>::new();
        let mut are_events_processed = vec![false; events.len()];
        let mut are_from_in_to_out = vec![false; events.len()];
        let mut contours_ids = vec![UNDEFINED_INDEX; events.len()];
        let mut contours_vertices = Vec::<Vec<&Point<Scalar>>>::new();
        let mut visited_endpoints_positions =
            vec![UNDEFINED_INDEX; self.to_unique_visited_endpoints_count()];
        for (event_id, &event) in events.iter().enumerate() {
            if are_events_processed[event_id] {
                continue;
            }
            let contour_id = contours_vertices.len();
            self.compute_relations(
                event,
                contour_id,
                &mut are_internal,
                &mut depths,
                &mut holes,
                &mut parents,
                &are_from_in_to_out,
                &contours_ids,
                &events_ids,
            );
            let contour_events = self.to_contour_events(
                event,
                &events,
                &events_ids,
                &connectivity,
                &are_events_processed,
                &mut visited_endpoints_positions,
            );
            self.process_contour_events(
                &contour_events,
                contour_id,
                &mut are_events_processed,
                &mut are_from_in_to_out,
                &mut contours_ids,
                &events_ids,
            );
            let mut vertices =
                self.contour_events_to_vertices(&contour_events);
            if depths[contour_id].is_odd() {
                vertices[1..].reverse();
            }
            contours_vertices.push(vertices);
        }
        let mut result = Vec::with_capacity(contours_vertices.len());
        for (contour_id, contour_vertices) in
            contours_vertices.iter().enumerate()
        {
            if are_internal[contour_id] {
                // hole of a hole is an external polygon
                result.extend(holes[contour_id].iter().map(|&hole_id| {
                    Polygon::from((
                        Contour::from(collect_references(
                            &contours_vertices[hole_id],
                        )),
                        holes[hole_id]
                            .iter()
                            .map(|&hole_hole_id| {
                                Contour::from(collect_references(
                                    &contours_vertices[hole_hole_id],
                                ))
                            })
                            .collect(),
                    ))
                }));
            } else {
                result.push(Polygon::from((
                    Contour::from(collect_references(contour_vertices)),
                    holes[contour_id]
                        .iter()
                        .map(|&hole_id| {
                            Contour::from(collect_references(
                                &contours_vertices[hole_id],
                            ))
                        })
                        .collect(),
                )));
            }
        }
        result
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
            self.below_event_from_result[event_position] = if !self
                .detect_if_left_event_from_result(below_event)
                || self.is_left_event_vertical(below_event)
            {
                self.below_event_from_result[below_event_position]
            } else {
                below_event
            };
        }
        self.are_from_result[event_position] =
            self.detect_if_left_event_from_result(event);
    }

    #[allow(clippy::too_many_arguments)]
    fn compute_relations(
        &self,
        event: Event,
        contour_id: usize,
        are_internal: &mut Vec<bool>,
        depths: &mut Vec<usize>,
        holes: &mut Vec<Vec<usize>>,
        parents: &mut Vec<usize>,
        are_from_in_to_out: &[bool],
        contours_ids: &[usize],
        events_ids: &[usize],
    ) {
        debug_assert!(is_event_left(event));
        let mut depth = 0;
        let mut parent = UNDEFINED_INDEX;
        let mut is_internal = false;
        let below_event_from_result =
            self.below_event_from_result[left_event_to_position(event)];
        if below_event_from_result != UNDEFINED_EVENT {
            let below_event_from_result_id =
                events_ids[below_event_from_result];
            let below_contour_id = contours_ids[below_event_from_result_id];
            if !are_from_in_to_out[below_event_from_result_id] {
                if !are_internal[below_contour_id] {
                    holes[below_contour_id].push(contour_id);
                    parent = below_contour_id;
                    depth = depths[below_contour_id] + 1;
                    is_internal = true;
                }
            } else if are_internal[below_contour_id] {
                let below_contour_parent = parents[below_contour_id];
                holes[below_contour_parent].push(contour_id);
                parent = below_contour_parent;
                depth = depths[below_contour_id];
                is_internal = true;
            }
        }
        holes.push(vec![]);
        parents.push(parent);
        depths.push(depth);
        are_internal.push(is_internal);
    }

    fn contour_events_to_vertices(&self, events: &[Event]) -> Vec<&Point>
    where
        for<'a> &'a Point: Orient,
    {
        let mut result = Vec::with_capacity(events.len());
        result.push(self.get_event_start(events[0]));
        for &event in &events[..events.len() - 1] {
            result.push(self.get_event_end(event));
        }
        shrink_collinear_vertices(&result)
    }

    fn events_count(&self) -> usize {
        self.endpoints.len()
    }

    fn events_to_connectivity(&self, events: &[Event]) -> Vec<usize>
    where
        Point: PartialEq,
    {
        let events_count = events.len();
        let mut result = vec![0; events_count];
        let mut event_id = 0;
        while event_id < events_count {
            let current_start = self.get_event_start(events[event_id]);
            let right_start_event_id = event_id;
            while event_id < events_count
                && self.get_event_start(events[event_id]) == current_start
                && !is_event_left(events[event_id])
            {
                event_id += 1;
            }
            let left_start_event_id = event_id;
            while event_id < events_count
                && self.get_event_start(events[event_id]) == current_start
            {
                event_id += 1;
            }
            let left_stop_event_id = event_id - 1;
            let has_right_events = left_start_event_id > right_start_event_id;
            let has_left_events = left_stop_event_id >= left_start_event_id;
            if has_right_events {
                result.splice(
                    right_start_event_id..left_start_event_id - 1,
                    (right_start_event_id + 1)..left_start_event_id,
                );
                result[left_start_event_id - 1] = if has_left_events {
                    left_stop_event_id
                } else {
                    right_start_event_id
                };
            }
            if has_left_events {
                result[left_start_event_id] = if has_right_events {
                    right_start_event_id
                } else {
                    left_stop_event_id
                };
                result.splice(
                    (left_start_event_id + 1)..=left_stop_event_id,
                    left_start_event_id..left_stop_event_id,
                );
            }
        }
        result
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

    fn is_left_event_common_polyline_component(&self, event: Event) -> bool {
        self.overlap_kinds[left_event_to_position(event)]
            == OverlapKind::DifferentOrientation
    }

    fn is_left_event_common_region_boundary(&self, event: Event) -> bool {
        self.overlap_kinds[left_event_to_position(event)]
            == OverlapKind::SameOrientation
    }

    fn is_left_event_from_first_operand(&self, event: Event) -> bool {
        self.left_event_to_segment_id(event) < self.first_segments_count
    }

    fn is_left_event_inside(&self, event: Event) -> bool {
        let event_position = left_event_to_position(event);
        self.other_have_interior_to_left[event_position]
            && self.overlap_kinds[event_position] == OverlapKind::None
    }

    fn is_left_event_outside(&self, event: Event) -> bool {
        let event_position = left_event_to_position(event);
        !self.other_have_interior_to_left[event_position]
            && self.overlap_kinds[event_position] == OverlapKind::None
    }

    fn is_left_event_overlapping(&self, event: Event) -> bool {
        self.overlap_kinds[left_event_to_position(event)] != OverlapKind::None
    }

    fn is_left_event_vertical(&self, event: Event) -> bool
    where
        for<'a> &'a Point: Elemental,
        for<'a> <&'a Point as Elemental>::Coordinate: PartialEq,
    {
        debug_assert!(is_event_left(event));
        self.get_event_start(event).x() == self.get_event_end(event).x()
    }

    fn left_event_to_segment_id(&self, event: Event) -> usize {
        self.segments_ids[left_event_to_position(event)]
    }

    fn process_contour_events(
        &self,
        contour_events: &[Event],
        contour_id: usize,
        are_events_processed: &mut [bool],
        are_from_in_to_out: &mut [bool],
        contours_ids: &mut [usize],
        events_ids: &[usize],
    ) {
        for &event in contour_events {
            are_events_processed[events_ids[event]] = true;
            are_events_processed[events_ids[self.to_opposite_event(event)]] =
                true;
            if is_event_left(event) {
                are_from_in_to_out[events_ids[event]] = false;
                contours_ids[events_ids[event]] = contour_id;
            } else {
                are_from_in_to_out
                    [events_ids[self.to_opposite_event(event)]] = true;
                contours_ids[events_ids[self.to_opposite_event(event)]] =
                    contour_id;
            }
        }
    }

    fn to_contour_events(
        &self,
        event: Event,
        events: &[Event],
        events_ids: &[usize],
        connectivity: &[usize],
        are_events_processed: &[bool],
        visited_endpoints_positions: &mut [usize],
    ) -> Vec<Event>
    where
        Point: PartialEq,
    {
        debug_assert!(is_event_left(event));
        let mut result = vec![event];
        visited_endpoints_positions[self.to_start_id(event)] = 0;
        let mut opposite_event_id = events_ids[self.to_opposite_event(event)];
        debug_assert_ne!(opposite_event_id, UNDEFINED_INDEX);
        let mut cursor = event;
        let contour_start = self.get_event_start(event);
        let mut visited_endpoints_ids = vec![self.to_start_id(event)];
        while self.get_event_end(cursor) != contour_start {
            let previous_endpoint_position =
                visited_endpoints_positions[self.to_end_id(cursor)];
            if previous_endpoint_position == UNDEFINED_INDEX {
                visited_endpoints_positions[self.to_end_id(cursor)] =
                    result.len();
            } else {
                // vertices loop found, i.e. contour has self-intersection
                debug_assert_ne!(previous_endpoint_position, 0);
                result.drain(previous_endpoint_position..);
            }
            visited_endpoints_ids.push(self.to_end_id(cursor));
            let event_id = to_next_event_id(
                opposite_event_id,
                are_events_processed,
                connectivity,
            );
            if event_id == UNDEFINED_INDEX {
                break;
            }
            cursor = events[event_id];
            opposite_event_id = events_ids[self.to_opposite_event(cursor)];
            result.push(cursor);
        }
        for endpoint_id in visited_endpoints_ids {
            visited_endpoints_positions[endpoint_id] = UNDEFINED_INDEX;
        }
        debug_assert!(visited_endpoints_positions
            .iter()
            .all(|&position| position == UNDEFINED_INDEX));
        result
    }

    fn to_end_id(&self, event: Event) -> usize {
        self.starts_ids[self.to_opposite_event(event)]
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

    fn to_start_id(&self, event: Event) -> usize {
        self.starts_ids[event]
    }

    fn to_sweep_line_key(&self, event: Event) -> SweepLineKey<Point> {
        SweepLineKey::new(
            event,
            self.is_left_event_from_first_operand(event),
            &self.endpoints,
            &self.opposites,
        )
    }

    fn to_unique_visited_endpoints_count(&self) -> usize {
        self.current_endpoint_id + 1
    }
}

fn to_next_event_id(
    event_id: usize,
    are_events_processed: &[bool],
    connectivity: &[usize],
) -> usize {
    let mut candidate = event_id;
    loop {
        candidate = connectivity[candidate];
        if !are_events_processed[candidate] {
            return candidate;
        } else if candidate == event_id {
            return UNDEFINED_INDEX;
        }
    }
}

impl<Point: Clone + PartialOrd, const KIND: u8> Operation<Point, KIND>
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

impl<Point: Clone, const KIND: u8> Operation<Point, KIND> {
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
        self.below_event_from_result.push(UNDEFINED_EVENT);
        self.overlap_kinds.push(OverlapKind::None);
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
            below_event_from_result: vec![UNDEFINED_EVENT; segments_count],
            current_endpoint_first_event: UNDEFINED_EVENT,
            current_endpoint_id: 0,
            endpoints: Box::new(Vec::with_capacity(initial_events_count)),
            events_queue_data: BinaryHeap::with_capacity(initial_events_count),
            have_interior_to_left: vec![true; segments_count],
            opposites: Box::new(Vec::with_capacity(initial_events_count)),
            other_have_interior_to_left: vec![false; segments_count],
            overlap_kinds: vec![OverlapKind::None; segments_count],
            segments_ids: (0..segments_count).collect(),
            starts_ids: vec![UNDEFINED_INDEX; initial_events_count],
            sweep_line_data: BTreeSet::new(),
        }
    }
}

fn collect_references<T: Clone>(vertices: &[&T]) -> Vec<T> {
    vertices.iter().copied().cloned().collect()
}
