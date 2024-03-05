use crate::bounded::{Bounded, Box};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_event_right, linear, mixed, Event, DIFFERENCE};
use crate::geometries::{
    Contour, Empty, Multipolygon, Point, Polygon, Segment,
};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_false_indices,
    flags_to_true_indices, to_boxes_have_common_continuum,
    to_boxes_ids_with_common_continuum, to_sorted_pair,
    IntersectCrossingSegments, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relatable;
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
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: Self) -> Self::Output {
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
        let other_bounding_boxes = other
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_continuum_segments_ids =
            to_boxes_ids_with_common_continuum(
                &other_bounding_boxes,
                &bounding_box,
            );
        if other_common_continuum_segments_ids.is_empty() {
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
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other.segments[index])
                .collect::<Vec<_>>();
        let mut operation = linear::Operation::<Point<_>, DIFFERENCE>::from((
            &common_continuum_segments,
            &other_common_continuum_segments,
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
            if is_event_right(event) {
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

impl<Scalar> Difference<&Contour<Scalar>> for &Multisegment<Scalar>
where
    Scalar: Clone + Ord,
    linear::Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Contour<Scalar>) -> Self::Output {
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
        let other_segments = other.segments();
        let other_bounding_boxes = other_segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_continuum_segments_ids =
            to_boxes_ids_with_common_continuum(
                &other_bounding_boxes,
                &bounding_box,
            );
        if other_common_continuum_segments_ids.is_empty() {
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
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>();
        let mut operation = linear::Operation::<Point<_>, DIFFERENCE>::from((
            &common_continuum_segments,
            &other_common_continuum_segments,
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
            if is_event_right(event) {
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

impl<Scalar: Ord> Difference<&Multipolygon<Scalar>> for &Multisegment<Scalar>
where
    mixed::Operation<Point<Scalar>, true, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
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
            if is_event_right(event) {
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
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
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
            if is_event_right(event) {
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
    Point<Scalar>: Clone + PartialOrd,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
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
        let (other_start, other_end) = to_sorted_pair(other.endpoints());
        for (index, segment) in self.segments.iter().enumerate() {
            if segment.to_bounding_box().disjoint_with(&other_bounding_box) {
                result.push(segment.clone());
                continue;
            }
            let (start, end) = to_sorted_pair(segment.endpoints());
            if other_start == start && other_end == end {
                result.extend(self.segments[index + 1..].iter().cloned());
                break;
            }
            let start_orientation = other_end.orient(other_start, start);
            let end_orientation = other_end.orient(other_start, end);
            if start_orientation != Orientation::Collinear
                && end_orientation != Orientation::Collinear
                && start_orientation != end_orientation
            {
                let other_start_orientation = start.orient(end, other_start);
                let other_end_orientation = start.orient(end, other_end);
                if other_start_orientation != Orientation::Collinear
                    && other_end_orientation != Orientation::Collinear
                    && (other_start_orientation != other_end_orientation)
                {
                    let cross_point =
                        IntersectCrossingSegments::intersect_crossing_segments(
                            start,
                            end,
                            other_start,
                            other_end,
                        );
                    result.push(Segment::new(
                        start.clone(),
                        cross_point.clone(),
                    ));
                    result.push(Segment::new(cross_point, end.clone()));
                    continue;
                }
            } else if start_orientation == Orientation::Collinear
                && end_orientation == Orientation::Collinear
            {
                if other_start == start {
                    if other_end < end {
                        result
                            .push(Segment::new(other_end.clone(), end.clone()))
                    }
                    continue;
                } else if other_end == end {
                    if start < other_start {
                        result.push(Segment::new(
                            start.clone(),
                            other_start.clone(),
                        ));
                    }
                    continue;
                } else if other_start < start && start < other_end {
                    if other_end < end {
                        result.push(Segment::new(
                            other_end.clone(),
                            end.clone(),
                        ));
                    }
                    continue;
                } else if start < other_start && other_start < end {
                    result.push(Segment::new(
                        start.clone(),
                        other_start.clone(),
                    ));
                    if other_end < end {
                        result.push(Segment::new(
                            other_end.clone(),
                            end.clone(),
                        ));
                    }
                    continue;
                }
            }
            result.push(segment.clone());
        }
        result
    }
}
