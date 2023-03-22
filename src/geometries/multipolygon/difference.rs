use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, DIFFERENCE};
use crate::geometries::{Empty, Point, Polygon};
use crate::operations::{
    do_boxes_have_no_common_area, flags_to_false_indices, flags_to_true_indices, merge_boxes,
    to_boxes_have_common_area_with_box, to_boxes_ids_with_common_area_with_box,
};
use crate::relatable::Relatable;
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

impl<Digit, const SHIFT: usize> Difference for &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SHIFT>>>,
            DIFFERENCE,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        >,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, DIFFERENCE>: From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
        )> + Iterator<Item = Event>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>> + Clone,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.polygons.clone();
        }
        let boxes_have_common_area =
            to_boxes_have_common_area_with_box(&bounding_boxes, &other_bounding_box);
        let common_area_polygons_ids = flags_to_true_indices(&boxes_have_common_area);
        if common_area_polygons_ids.is_empty() {
            return self.polygons.clone();
        }
        let other_common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&other_bounding_boxes, &bounding_box);
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
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            events.push(event);
        }
        let mut result = Multipolygon::<_>::reduce_events(events, &mut operation);
        result.reserve(self.polygons.len() - common_area_polygons.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result
    }
}

impl<Digit, const SHIFT: usize> Difference<&Polygon<Fraction<BigInt<Digit, SHIFT>>>>
    for &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SHIFT>>>,
            DIFFERENCE,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        >,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, DIFFERENCE>: From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
        )> + Iterator<Item = Event>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>> + Clone,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn difference(self, other: &Polygon<Fraction<BigInt<Digit, SHIFT>>>) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return self.polygons.clone();
        }
        let boxes_have_common_area =
            to_boxes_have_common_area_with_box(&bounding_boxes, &other_bounding_box);
        let common_area_polygons_ids = flags_to_true_indices(&boxes_have_common_area);
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
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((&common_area_polygons, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            events.push(event);
        }
        let mut result = Multipolygon::<_>::reduce_events(events, &mut operation);
        result.reserve(self.polygons.len() - common_area_polygons.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_area)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result
    }
}

impl<Digit, const SHIFT: usize> Difference<&Multipolygon<Fraction<BigInt<Digit, SHIFT>>>>
    for &Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, DIFFERENCE>: From<(
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
        )> + Iterator<Item = Event>,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>>
        + Clone
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SHIFT>>>,
            DIFFERENCE,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        >,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn difference(self, other: &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>) -> Self::Output {
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![self.clone()];
        }
        let other_common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&other_bounding_boxes, &other_bounding_box);
        if other_common_area_polygons_ids.is_empty() {
            return vec![self.clone()];
        }
        let max_x = bounding_box.get_max_x();
        let other_common_area_polygons = other_common_area_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation =
            Operation::<Point<_>, DIFFERENCE>::from((self, &other_common_area_polygons));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            events.push(event);
        }
        Polygon::<_>::reduce_events(events, &mut operation)
    }
}
