use crate::bounded::{Bounded, Box};
use crate::clipping::linear::{intersect_segment_with_segments, Operation};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_right_event, Event, INTERSECTION};
use crate::geometries::{Empty, Point, Segment};
use crate::operations::{
    do_boxes_have_no_common_continuum, to_boxes_ids_with_common_continuum,
    Orient,
};
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{Elemental, Intersection};

use super::types::Multisegment;

impl<Scalar> Intersection<Empty> for Multisegment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Multisegment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Multisegment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Multisegment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection for &Multisegment<Scalar>
where
    Scalar: Ord,
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![];
        }
        let bounding_boxes = self
            .segments
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            &bounding_boxes,
            &other_bounding_box,
        );
        if common_continuum_segments_ids.is_empty() {
            return vec![];
        } else if common_continuum_segments_ids.len() == 1 {
            return intersect_segment_with_segments(
                &self.segments[common_continuum_segments_ids[0]],
                other.segments.iter(),
            );
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
            return vec![];
        }
        let common_continuum_segments = common_continuum_segments_ids
            .iter()
            .map(|&index| &self.segments[index]);
        if other_common_continuum_segments_ids.len() == 1 {
            return intersect_segment_with_segments(
                &other.segments[other_common_continuum_segments_ids[0]],
                common_continuum_segments,
            );
        }
        let common_continuum_segments =
            common_continuum_segments.collect::<Vec<_>>();
        let other_common_continuum_segments =
            other_common_continuum_segments_ids
                .iter()
                .map(|&index| &other.segments[index])
                .collect::<Vec<_>>();
        let min_max_x = unsafe {
            common_continuum_segments_ids
                .into_iter()
                .map(|index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(unsafe {
            other_common_continuum_segments_ids
                .into_iter()
                .map(|index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let mut operation = Operation::<Point<_>, INTERSECTION>::from((
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
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            if is_right_event(event) {
                events.push(operation.to_opposite_event(event));
            }
        }
        operation.reduce_events(events)
    }
}

impl<Scalar> Intersection<&Segment<Scalar>> for &Multisegment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Segment<Scalar>) -> Self::Output {
        intersect_segment_with_segments(other, self.segments.iter())
    }
}

impl<Scalar> Intersection<&Multisegment<Scalar>> for &Segment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Multisegment<Scalar>) -> Self::Output {
        intersect_segment_with_segments(self, other.segments.iter())
    }
}
