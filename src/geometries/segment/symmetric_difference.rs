use crate::bounded::{Bounded, Box};
use crate::clipping::linear::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_event_right, Event, SYMMETRIC_DIFFERENCE};
use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_false_indices,
    flags_to_true_indices, to_boxes_have_common_continuum, to_sorted_pair,
    IntersectCrossingSegments, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::traits::{
    Elemental, Iterable, Lengthsome, Multisegmental, SymmetricDifference,
};

use super::types::Segment;

impl<Scalar> SymmetricDifference<Empty> for Segment<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<&Empty> for Segment<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference<&Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
{
    type Output = Vec<Segment<Scalar>>;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        let (start, end) = to_sorted_pair((&self.start, &self.end));
        let (other_start, other_end) =
            to_sorted_pair((&other.start, &other.end));
        if start == other_start && end == other_end {
            return vec![];
        }
        let other_start_orientation = end.orient(start, other_start);
        let other_end_orientation = end.orient(start, other_end);
        if other_start_orientation != Orientation::Collinear
            && other_end_orientation != Orientation::Collinear
            && other_start_orientation != other_end_orientation
        {
            let start_orientation = other_start.orient(other_end, start);
            let end_orientation = other_start.orient(other_end, end);
            if start_orientation != Orientation::Collinear
                && end_orientation != Orientation::Collinear
                && start_orientation != end_orientation
            {
                let cross_point =
                    IntersectCrossingSegments::intersect_crossing_segments(
                        start,
                        end,
                        other_start,
                        other_end,
                    );
                return vec![
                    Segment::new(start.clone(), cross_point.clone()),
                    Segment::new(cross_point.clone(), end.clone()),
                    Segment::new(other_start.clone(), cross_point.clone()),
                    Segment::new(cross_point, other_end.clone()),
                ];
            }
        } else if other_start_orientation == Orientation::Collinear
            && other_end_orientation == Orientation::Collinear
        {
            if start == other_start {
                return vec![Segment::new(end.clone(), other_end.clone())];
            } else if end == other_end {
                return vec![Segment::new(start.clone(), other_start.clone())];
            } else if other_end == start {
                return vec![Segment::new(other_start.clone(), end.clone())];
            } else if end == other_start {
                return vec![Segment::new(start.clone(), other_end.clone())];
            } else if other_start < end && start < other_end {
                return vec![
                    Segment::new(start.clone(), other_start.clone()),
                    Segment::new(other_end.clone(), end.clone()),
                ];
            }
        }
        vec![self.clone(), other.clone()]
    }
}

impl<Scalar> SymmetricDifference<&Contour<Scalar>> for &Segment<Scalar>
where
    Scalar: Clone + Ord,
    Segment<Scalar>: Clone,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn symmetric_difference(self, other: &Contour<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        let other_segments = other.segments();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = Vec::with_capacity(1 + other_segments.len());
            result.push(self.clone());
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
            let mut result = Vec::with_capacity(1 + other_segments.len());
            result.push(self.clone());
            result.extend(other_segments.iter().cloned());
            return result;
        }
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (self, &other_common_continuum_segments),
        );
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if is_event_right(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            other_segments.len() - other_common_continuum_segments.len(),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other_segments[index].clone()),
        );
        result
    }
}

impl<Scalar> SymmetricDifference<&Multisegment<Scalar>> for &Segment<Scalar>
where
    Scalar: Clone + Ord,
    Segment<Scalar>: Clone,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn symmetric_difference(
        self,
        other: &Multisegment<Scalar>,
    ) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        let other_segments = other.segments();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = Vec::with_capacity(1 + other_segments.len());
            result.push(self.clone());
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
            let mut result = Vec::with_capacity(1 + other_segments.len());
            result.push(self.clone());
            result.extend(other_segments.iter().cloned());
            return result;
        }
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (self, &other_common_continuum_segments),
        );
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if is_event_right(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            other_segments.len() - other_common_continuum_segments.len(),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other_segments[index].clone()),
        );
        result
    }
}
