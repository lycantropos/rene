use crate::bounded::{Bounded, Box};
use crate::clipping::linear::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_right_event, Event, UNION};
use crate::geometries::{Empty, Multisegment, Point, Segment};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_false_indices,
    flags_to_true_indices, to_boxes_have_common_continuum, to_sorted_pair,
    IntersectCrossingSegments, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::traits::{
    Elemental, Iterable, Lengthsome, Multisegmental, Segmental, Union,
};

use super::types::Contour;

impl<Scalar> Union<Empty> for Contour<Scalar> {
    type Output = Self;

    fn union(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<&Empty> for Contour<Scalar> {
    type Output = Self;

    fn union(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<Empty> for &Contour<Scalar>
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn union(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Union<&Empty> for &Contour<Scalar>
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn union(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Union for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    Contour<Scalar>: Clone,
    Operation<Point<Scalar>, UNION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.segments.clone();
            result.extend_from_slice(&other.segments);
            return result;
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
            let mut result = self.segments.clone();
            result.extend_from_slice(&other.segments);
            return result;
        }
        let other_bounding_boxes = other
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_bounding_boxes,
            &bounding_box,
        );
        let other_common_continuum_segments_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_segments_ids.is_empty() {
            let mut result = self.segments.clone();
            result.extend_from_slice(&other.segments);
            return result;
        }
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other.segments[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, UNION>::from((
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
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            (self.segments.len() - common_continuum_segments.len())
                + (other.segments.len()
                    - other_common_continuum_segments.len()),
        );
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other.segments[index].clone()),
        );
        result
    }
}

impl<Scalar> Union<&Multisegment<Scalar>> for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    Contour<Scalar>: Clone,
    Operation<Point<Scalar>, UNION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Multisegment<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Multisegment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        let other_segments = other.segments();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.segments.clone();
            result.extend(other_segments.iter().cloned());
            return result;
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
            let mut result = self.segments.clone();
            result.extend(other_segments.iter().cloned());
            return result;
        }
        let other_bounding_boxes = other_segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_bounding_boxes,
            &bounding_box,
        );
        let other_common_continuum_segments_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_segments_ids.is_empty() {
            let mut result = self.segments.clone();
            result.extend(other_segments.iter().cloned());
            return result;
        }
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, UNION>::from((
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
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            (self.segments.len() - common_continuum_segments.len())
                + (other_segments.len()
                    - other_common_continuum_segments.len()),
        );
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other_segments[index].clone()),
        );
        result
    }
}

impl<Scalar: PartialEq> Union<&Segment<Scalar>> for &Contour<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Segment<Scalar>) -> Self::Output {
        let (bounding_box, other_bounding_box) =
            (self.to_bounding_box(), other.to_bounding_box());
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.segments.clone();
            result.push(other.clone());
            return result;
        }
        let segments = &self.segments;
        let mut result = Vec::with_capacity(segments.len());
        let mut other_break_points = vec![];
        let (other_start, other_end) = to_sorted_pair(other.endpoints());
        for (index, segment) in segments.iter().enumerate() {
            if segment.to_bounding_box().disjoint_with(&other_bounding_box) {
                result.push(segment.clone());
                continue;
            }
            let (start, end) = to_sorted_pair(segment.endpoints());
            if other_start == start && other_end == end {
                result.extend(segments[index + 1..].iter().cloned());
                break;
            }
            let start_orientation = other_end.orient(other_start, start);
            let end_orientation = other_end.orient(other_start, end);
            if start_orientation == end_orientation {
                if start_orientation == Orientation::Collinear {
                    if other_start == start {
                        if other_end < end {
                            result.push(Segment::new(
                                other_end.clone(),
                                end.clone(),
                            ));
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
            } else if start_orientation == Orientation::Collinear {
                if other_start < start && start < other_end {
                    other_break_points.push(start.clone());
                }
            } else if end_orientation == Orientation::Collinear {
                if other_start < end && end < other_end {
                    other_break_points.push(end.clone());
                }
            } else {
                let other_start_orientation = start.orient(end, other_start);
                let other_end_orientation = start.orient(end, other_end);
                if other_start_orientation == Orientation::Collinear {
                    if start < other_start && other_start < end {
                        result.push(Segment::new(
                            start.clone(),
                            other_start.clone(),
                        ));
                        result.push(Segment::new(
                            other_start.clone(),
                            end.clone(),
                        ));
                        continue;
                    }
                } else if other_end_orientation == Orientation::Collinear {
                    if start < other_end && other_end < end {
                        result.push(Segment::new(
                            start.clone(),
                            other_end.clone(),
                        ));
                        result.push(Segment::new(
                            other_end.clone(),
                            end.clone(),
                        ));
                        continue;
                    }
                } else if other_start_orientation != other_end_orientation {
                    let cross_point =
                        IntersectCrossingSegments::intersect_crossing_segments(
                            start,
                            end,
                            other_start,
                            other_end,
                        );
                    other_break_points.push(cross_point.clone());
                    result.push(Segment::new(
                        start.clone(),
                        cross_point.clone(),
                    ));
                    result.push(Segment::new(cross_point, end.clone()));
                    continue;
                }
            }
            result.push(segment.clone());
        }
        if !other_break_points.is_empty() {
            other_break_points.sort();
            other_break_points.dedup();
            let mut start = other_start.clone();
            for end in other_break_points {
                result.push(Segment::new(start, end.clone()));
                start = end;
            }
            result.push(Segment::new(start, other_end.clone()));
        } else {
            result.push(other.clone());
        }
        result
    }
}
