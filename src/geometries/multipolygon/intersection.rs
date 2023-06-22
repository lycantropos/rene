use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, INTERSECTION};
use crate::geometries::{Empty, Point, Polygon};
use crate::operations::{
    do_boxes_have_no_common_area, merge_boxes, to_boxes_ids_with_common_area_with_box,
};
use crate::relatable::Relatable;
use crate::traits::{Elemental, Intersection};

use super::types::Multipolygon;

impl<Scalar> Intersection<Empty> for Multipolygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Multipolygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Multipolygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Multipolygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Digit, const SHIFT: usize> Intersection for &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: ReduceEvents<
        Point<Fraction<BigInt<Digit, SHIFT>>>,
        INTERSECTION,
        Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
    >,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, INTERSECTION>: Iterator<Item = Event>
        + for<'a> From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
        )>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Box<&'a Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn intersection(self, other: Self) -> Self::Output {
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
            return vec![];
        }
        let common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&bounding_boxes, &other_bounding_box);
        if common_area_polygons_ids.is_empty() {
            return vec![];
        }
        let other_common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&other_bounding_boxes, &bounding_box);
        if other_common_area_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = unsafe {
            common_area_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(unsafe {
            other_common_area_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let common_area_polygons = common_area_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let other_common_area_polygons = other_common_area_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, INTERSECTION>::from((
            &common_area_polygons,
            &other_common_area_polygons,
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event);
        }
        Self::reduce_events(events, &mut operation)
    }
}

impl<Digit, const SHIFT: usize> Intersection<&Polygon<Fraction<BigInt<Digit, SHIFT>>>>
    for &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: ReduceEvents<
        Point<Fraction<BigInt<Digit, SHIFT>>>,
        INTERSECTION,
        Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
    >,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, INTERSECTION>: Iterator<Item = Event>
        + for<'a> From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
        )>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Box<&'a Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn intersection(self, other: &Polygon<Fraction<BigInt<Digit, SHIFT>>>) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![];
        }
        let common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&bounding_boxes, &other_bounding_box);
        if common_area_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = unsafe {
            common_area_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(other_bounding_box.get_max_x());
        let common_area_polygons = common_area_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let mut operation =
            Operation::<Point<_>, INTERSECTION>::from((&common_area_polygons, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event);
        }
        Self::reduce_events(events, &mut operation)
    }
}

impl<Digit, const SHIFT: usize> Intersection<&Multipolygon<Fraction<BigInt<Digit, SHIFT>>>>
    for &Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: ReduceEvents<
        Point<Fraction<BigInt<Digit, SHIFT>>>,
        INTERSECTION,
        Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
    >,
    Fraction<BigInt<Digit, SHIFT>>: Clone + Ord,
    Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, INTERSECTION>: Iterator<Item = Event>
        + for<'a> From<(
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
            &'a [&'a Polygon<Fraction<BigInt<Digit, SHIFT>>>],
        )>,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Box<&'a Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
    for<'a> &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn intersection(self, other: &Multipolygon<Fraction<BigInt<Digit, SHIFT>>>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![];
        }
        let other_common_area_polygons_ids =
            to_boxes_ids_with_common_area_with_box(&other_bounding_boxes, &bounding_box);
        if other_common_area_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = bounding_box.get_max_x().min(unsafe {
            other_common_area_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let other_common_area_polygons = other_common_area_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation =
            Operation::<Point<_>, INTERSECTION>::from((self, &other_common_area_polygons));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event);
        }
        Self::reduce_events(events, &mut operation)
    }
}
