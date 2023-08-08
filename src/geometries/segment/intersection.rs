use crate::bounded::{Bounded, Box};
use crate::clipping::linear::{
    intersect_segment_with_common_continuum_bounding_box_segment,
    intersect_segment_with_segments,
};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::INTERSECTION;
use crate::clipping::{mixed, Event};
use crate::geometries::{
    Contour, Empty, Multipolygon, Multisegment, Point, Polygon,
};
use crate::operations::{
    do_boxes_have_no_common_continuum, to_boxes_ids_with_common_continuum,
    Orient,
};
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{
    Elemental, Intersection, Iterable, Multipolygonal, Multisegmental,
    Segmental,
};

use super::types::Segment;

impl<Scalar> Intersection<Empty> for Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection for &Segment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Option<Segment<Scalar>>;

    fn intersection(self, other: Self) -> Self::Output {
        if do_boxes_have_no_common_continuum(
            &self.to_bounding_box(),
            &other.to_bounding_box(),
        ) {
            None
        } else {
            let (start, end) = self.endpoints();
            let (other_start, other_end) = other.endpoints();
            intersect_segment_with_common_continuum_bounding_box_segment(
                start,
                end,
                other_start,
                other_end,
            )
            .map(|(start, end)| Segment::new(start.clone(), end.clone()))
        }
    }
}

impl<Scalar> Intersection<&Contour<Scalar>> for &Segment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Contour<Scalar>) -> Self::Output {
        intersect_segment_with_segments(self, other.segments().into_iter())
    }
}

impl<Scalar> Intersection<&Multipolygon<Scalar>> for &Segment<Scalar>
where
    Scalar: Clone + Ord,
    mixed::Operation<Point<Scalar>, true, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
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
        let min_max_x = bounding_box.get_max_x().min(unsafe {
            other_common_continuum_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let other_common_continuum_polygons =
            other_common_continuum_polygons_ids
                .into_iter()
                .map(|index| &other_polygons[index])
                .collect::<Vec<_>>();
        let mut operation =
            mixed::Operation::<Point<_>, true, INTERSECTION>::from((
                self,
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
            events.push(event);
        }
        operation.reduce_events(events)
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
        intersect_segment_with_segments(self, other.segments().into_iter())
    }
}

impl<Scalar> Intersection<&Polygon<Scalar>> for &Segment<Scalar>
where
    Scalar: Clone + Ord,
    mixed::Operation<Point<Scalar>, true, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a Polygon<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
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
        let min_max_x =
            bounding_box.get_max_x().min(other_bounding_box.get_max_x());
        let mut operation =
            mixed::Operation::<Point<_>, true, INTERSECTION>::from((
                self, other,
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
            events.push(event);
        }
        operation.reduce_events(events)
    }
}
