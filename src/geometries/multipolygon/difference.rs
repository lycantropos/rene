use crate::bounded::{Bounded, Box};
use crate::clipping::shaped::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, DIFFERENCE};
use crate::geometries::{Empty, Point, Polygon};
use crate::operations::{
    do_boxes_have_no_common_area, flags_to_false_indices,
    flags_to_true_indices, to_boxes_have_common_area,
    to_boxes_ids_with_common_area,
};
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{Difference, Elemental};

use super::types::Multipolygon;

impl<Scalar> Difference<Empty> for Multipolygon<Scalar> {
    type Output = Self;

    fn difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Empty> for Multipolygon<Scalar> {
    type Output = Self;

    fn difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Empty> for &Multipolygon<Scalar>
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference<&Empty> for &Multipolygon<Scalar>
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference for &Multipolygon<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a [&'a Polygon<Scalar>], &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.polygons.clone();
        }
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_area =
            to_boxes_have_common_area(&bounding_boxes, &other_bounding_box);
        let common_area_polygons_ids =
            flags_to_true_indices(&boxes_have_common_area);
        if common_area_polygons_ids.is_empty() {
            return self.polygons.clone();
        }
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_area_polygons_ids = to_boxes_ids_with_common_area(
            &other_bounding_boxes,
            &bounding_box,
        );
        if other_common_area_polygons_ids.is_empty() {
            return self.polygons.clone();
        }
        let max_x = unsafe {
            common_area_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_area_polygons = common_area_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let other_common_area_polygons = other_common_area_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((
            &common_area_polygons,
            &other_common_area_polygons,
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
            events.push(event);
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.polygons.len() - common_area_polygons.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result
    }
}

impl<Scalar> Difference<&Polygon<Scalar>> for &Multipolygon<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a [&'a Polygon<Scalar>], &'a Polygon<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn difference(self, other: &Polygon<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.polygons.clone();
        }
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let boxes_have_common_area =
            to_boxes_have_common_area(&bounding_boxes, &other_bounding_box);
        let common_area_polygons_ids =
            flags_to_true_indices(&boxes_have_common_area);
        if common_area_polygons_ids.is_empty() {
            return self.polygons.clone();
        }
        let max_x = unsafe {
            common_area_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        };
        let common_area_polygons = common_area_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((
            &common_area_polygons,
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
            events.push(event);
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.polygons.len() - common_area_polygons.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result
    }
}
