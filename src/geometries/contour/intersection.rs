use crate::bounded::{Bounded, Box};
use crate::clipping::linear::{
    intersect_segment_with_segments, intersect_segments_with_segments,
    Operation,
};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{is_event_left, mixed, Event, INTERSECTION};
use crate::geometries::{
    Empty, Multipolygon, Multisegment, Point, Polygon, Segment,
};
use crate::operations::{
    do_boxes_have_no_common_continuum, to_boxes_ids_with_common_continuum,
    Orient,
};
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{
    Elemental, Intersection, Iterable, Multipolygonal, Multisegmental,
};

use super::types::Contour;

impl<Scalar> Intersection<Empty> for Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar: Ord> Intersection for &Contour<Scalar>
where
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
        intersect_segments_with_segments(
            self.segments(),
            other.segments(),
            bounding_box,
            other_bounding_box,
        )
    }
}

impl<Scalar> Intersection<&Multipolygon<Scalar>> for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    mixed::Operation<Point<Scalar>, true, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Multipolygon<Scalar>) -> Self::Output {
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
            return vec![];
        }
        let min_max_x = unsafe {
            common_continuum_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(unsafe {
            other_common_continuum_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
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
            mixed::Operation::<Point<_>, true, INTERSECTION>::from((
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
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            if is_event_left(event) {
                events.push(event);
            }
        }
        operation.reduce_events(events)
    }
}

impl<Scalar: Ord> Intersection<&Multisegment<Scalar>> for &Contour<Scalar>
where
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Multisegment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![];
        }
        intersect_segments_with_segments(
            self.segments(),
            other.segments(),
            bounding_box,
            other_bounding_box,
        )
    }
}

impl<Scalar> Intersection<&Polygon<Scalar>> for &Contour<Scalar>
where
    Scalar: Clone + Ord,
    mixed::Operation<Point<Scalar>, true, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a Polygon<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Polygon<Scalar>) -> Self::Output {
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
        }
        let min_max_x = unsafe {
            common_continuum_segments_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(other_bounding_box.get_max_x());
        let common_continuum_segments = common_continuum_segments_ids
            .into_iter()
            .map(|index| &self.segments[index])
            .collect::<Vec<_>>();
        let mut operation =
            mixed::Operation::<Point<_>, true, INTERSECTION>::from((
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
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            if is_event_left(event) {
                events.push(event);
            }
        }
        operation.reduce_events(events)
    }
}

impl<Scalar> Intersection<&Segment<Scalar>> for &Contour<Scalar>
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
