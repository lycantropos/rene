use std::ops::{Add, Div, Mul, Sub};

use crate::bounded::{Bounded, Box};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_right_event, linear, mixed, Event, DIFFERENCE};
use crate::geometries::{
    Contour, Empty, Multipolygon, Point, Polygon, Segment,
};
use crate::operations::{
    do_boxes_have_no_common_area, do_boxes_have_no_common_continuum,
    flags_to_false_indices, flags_to_true_indices, subtract_segments_overlap,
    to_boxes_have_common_area, to_boxes_have_common_continuum,
    to_boxes_ids_with_common_area, to_boxes_ids_with_common_continuum,
    CrossMultiply, IntersectCrossingSegments, Orient,
};
use crate::relatable::{Relatable, Relation};
use crate::sweeping::traits::EventsContainer;
use crate::traits::{
    Difference, Elemental, Iterable, Multipolygonal, Multisegmental, Segmental,
};

use super::types::Multisegment;

impl<Scalar> Difference<Empty> for Multisegment<Scalar> {
    type Output = Self;

    fn difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Empty> for Multisegment<Scalar> {
    type Output = Self;

    fn difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Empty> for &Multisegment<Scalar>
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference<&Empty> for &Multisegment<Scalar>
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference for &Multisegment<Scalar>
where
    Scalar: Clone + Ord,
    linear::Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.segments.clone();
        }
        let bounding_boxes = self
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_area =
            to_boxes_have_common_area(&bounding_boxes, &other_bounding_box);
        let common_area_segments_ids =
            flags_to_true_indices(&boxes_have_common_area);
        if common_area_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let other_bounding_boxes = other
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_area_segments_ids = to_boxes_ids_with_common_area(
            &other_bounding_boxes,
            &bounding_box,
        );
        if other_common_area_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let max_x = unsafe {
            common_area_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_area_segments = common_area_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let other_common_area_segments = other_common_area_segments_ids
            .into_iter()
            .map(|index| &other.segments[index])
            .collect::<Vec<_>>();
        let mut operation = linear::Operation::<Point<_>, DIFFERENCE>::from((
            &common_area_segments,
            &other_common_area_segments,
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.segments.len() - common_area_segments.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result
    }
}

impl<Scalar> Difference<&Contour<Scalar>> for &Multisegment<Scalar>
where
    Scalar: Clone + Ord,
    linear::Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Contour<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.segments.clone();
        }
        let bounding_boxes = self
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_area =
            to_boxes_have_common_area(&bounding_boxes, &other_bounding_box);
        let common_area_segments_ids =
            flags_to_true_indices(&boxes_have_common_area);
        if common_area_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let other_segments = other.segments();
        let other_bounding_boxes = other_segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_area_segments_ids = to_boxes_ids_with_common_area(
            &other_bounding_boxes,
            &bounding_box,
        );
        if other_common_area_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let max_x = unsafe {
            common_area_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_area_segments = common_area_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let other_common_area_segments = other_common_area_segments_ids
            .into_iter()
            .map(|index| &other_segments[index])
            .collect::<Vec<_>>();
        let mut operation = linear::Operation::<Point<_>, DIFFERENCE>::from((
            &common_area_segments,
            &other_common_area_segments,
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.segments.len() - common_area_segments.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result
    }
}

impl<Scalar: Ord> Difference<&Multipolygon<Scalar>> for &Multisegment<Scalar>
where
    mixed::Operation<Point<Scalar>, true, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Multipolygon<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return self.segments.clone();
        }
        let bounding_boxes = self
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_continuum = to_boxes_have_common_continuum(
            &bounding_boxes,
            &other_bounding_box,
        );
        let common_continuum_segments_ids =
            flags_to_true_indices(&boxes_have_common_continuum);
        if common_continuum_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let other_polygons = other.polygons();
        let other_bounding_boxes = other_polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_continuum_polygons_ids =
            to_boxes_ids_with_common_continuum(
                &other_bounding_boxes,
                &bounding_box,
            );
        if other_common_continuum_polygons_ids.is_empty() {
            return self.segments.clone();
        }
        let max_x = unsafe {
            common_continuum_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let other_common_continuum_polygons =
            other_common_continuum_polygons_ids
                .into_iter()
                .map(|index| &other_polygons[index])
                .collect::<Vec<_>>();
        let mut operation =
            mixed::Operation::<Point<_>, true, DIFFERENCE>::from((
                &common_continuum_segments,
                &other_common_continuum_polygons,
            ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.segments.len() - common_continuum_segments.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result
    }
}

impl<Scalar: Ord> Difference<&Polygon<Scalar>> for &Multisegment<Scalar>
where
    mixed::Operation<Point<Scalar>, true, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a Polygon<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Polygon<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return self.segments.clone();
        }
        let bounding_boxes = self
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_continuum = to_boxes_have_common_continuum(
            &bounding_boxes,
            &other_bounding_box,
        );
        let common_continuum_segments_ids =
            flags_to_true_indices(&boxes_have_common_continuum);
        if common_continuum_segments_ids.is_empty() {
            return self.segments.clone();
        }
        let max_x = unsafe {
            common_continuum_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let mut operation =
            mixed::Operation::<Point<_>, true, DIFFERENCE>::from((
                &common_continuum_segments,
                other,
            ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.segments.len() - common_continuum_segments.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result
    }
}

impl<Scalar: PartialEq> Difference<&Segment<Scalar>> for &Multisegment<Scalar>
where
    Scalar: Add<Output = Scalar>
        + Div<Output = Scalar>
        + Mul<Output = Scalar>
        + Sub<Output = Scalar>
        + for<'a> Mul<&'a Scalar, Output = Scalar>,
    linear::Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a Segment<Scalar>)>,
    Point<Scalar>: Clone + Elemental<Coordinate = Scalar> + Ord + PartialOrd,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar> + Orient,
    for<'a> &'a Scalar: Add<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Segment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return self.segments.clone();
        }
        let mut result = Vec::with_capacity(self.segments.len());
        for (index, segment) in self.segments.iter().enumerate() {
            if segment.to_bounding_box().disjoint_with(&other_bounding_box) {
                result.push(segment.clone());
                continue;
            }
            match segment.relate_to(other) {
                Relation::Equal => {
                    result.extend(self.segments[index + 1..].iter().cloned());
                    break;
                }
                Relation::Composite => {
                    let [left_start, left_end, right_start, right_end] = {
                        let mut endpoints = [
                            segment.start(),
                            segment.end(),
                            other.start(),
                            other.end(),
                        ];
                        endpoints.sort();
                        endpoints
                    };
                    if left_start == left_end {
                        result.push(Segment::new(
                            right_start.clone(),
                            right_end.clone(),
                        ));
                    } else if right_start == right_end {
                        result.push(Segment::new(
                            left_start.clone(),
                            left_end.clone(),
                        ));
                    } else {
                        result.push(Segment::new(
                            left_start.clone(),
                            left_end.clone(),
                        ));
                        result.push(Segment::new(
                            right_start.clone(),
                            right_end.clone(),
                        ));
                    }
                    result.extend(self.segments[index + 1..].iter().cloned());
                    break;
                }
                Relation::Cross => {
                    let cross_point =
                        IntersectCrossingSegments::intersect_crossing_segments(
                            segment.start(),
                            segment.end(),
                            other.start(),
                            other.end(),
                        );
                    result.push(Segment::new(
                        segment.start().clone(),
                        cross_point.clone(),
                    ));
                    result.push(Segment::new(
                        cross_point,
                        segment.end().clone(),
                    ));
                }
                Relation::Overlap => {
                    let (start, end) = subtract_segments_overlap(
                        segment.start(),
                        segment.end(),
                        other.start(),
                        other.end(),
                    );
                    result.push(Segment::new(start.clone(), end.clone()));
                }
                Relation::Component => continue,
                _ => result.push(segment.clone()),
            }
        }
        result
    }
}
