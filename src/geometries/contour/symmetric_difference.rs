use crate::bounded::{Bounded, Box};
use crate::clipping::linear::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_event_right, Event, SYMMETRIC_DIFFERENCE};
use crate::geometries::{Empty, Multisegment, Point, Segment};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_false_indices,
    flags_to_true_indices, to_boxes_have_common_continuum,
};
use crate::relatable::Relatable;
use crate::traits::{
    Elemental, Iterable, Lengthsome, Multisegmental, SymmetricDifference,
};

use super::types::Contour;

impl<Scalar> SymmetricDifference<Empty> for Contour<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<&Empty> for Contour<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<Empty> for &Contour<Scalar>
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference<&Empty> for &Contour<Scalar>
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    Contour<Scalar>: Clone,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn symmetric_difference(self, other: Self) -> Self::Output {
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
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (&common_continuum_segments, &other_common_continuum_segments),
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

impl<Scalar> SymmetricDifference<&Multisegment<Scalar>> for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    Contour<Scalar>: Clone,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
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
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (&common_continuum_segments, &other_common_continuum_segments),
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

impl<Scalar> SymmetricDifference<&Segment<Scalar>> for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a Segment<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn symmetric_difference(self, other: &Segment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.segments.clone();
            result.push(other.clone());
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
            result.push(other.clone());
            return result;
        }
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (&common_continuum_segments, other),
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
        result.reserve(self.segments.len() - common_continuum_segments.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.segments[index].clone()),
        );
        result
    }
}
