use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, INTERSECTION};
use crate::geometries::{Empty, Point, Polygon};
use crate::operations::{boxes_ids_coupled_with_box, merge_boxes};
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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Intersection
    for &Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Relatable,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            INTERSECTION,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
        >,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>, INTERSECTION>: From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>],
            &'a [&'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>],
        )> + Iterator<Item = Event>,
    Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>;

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
        let coupled_polygons_ids = boxes_ids_coupled_with_box(&bounding_boxes, &other_bounding_box);
        if coupled_polygons_ids.is_empty() {
            return vec![];
        }
        let other_coupled_polygons_ids =
            boxes_ids_coupled_with_box(&other_bounding_boxes, &bounding_box);
        if other_coupled_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = unsafe {
            coupled_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(unsafe {
            other_coupled_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let coupled_polygons = coupled_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let other_coupled_polygons = other_coupled_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation =
            Operation::<Point<_>, INTERSECTION>::from((&coupled_polygons, &other_coupled_polygons));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event)
        }
        Multipolygon::<_>::reduce_events(events, &mut operation)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Intersection<&Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>
    for &Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Relatable,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            INTERSECTION,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
        >,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>, INTERSECTION>: From<(
            &'a [&'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>],
            &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
        )> + Iterator<Item = Event>,
    Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>;

    fn intersection(
        self,
        other: &Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    ) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = other.to_bounding_box();
        if bounding_box.disjoint_with(&other_bounding_box)
            || bounding_box.touches(&other_bounding_box)
        {
            return vec![];
        }
        let coupled_polygons_ids = boxes_ids_coupled_with_box(&bounding_boxes, &other_bounding_box);
        if coupled_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = unsafe {
            coupled_polygons_ids
                .iter()
                .map(|&index| bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        }
        .min(other_bounding_box.get_max_x());
        let coupled_polygons = coupled_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, INTERSECTION>::from((&coupled_polygons, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event)
        }
        Multipolygon::<_>::reduce_events(events, &mut operation)
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize>
    Intersection<&Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>
    for &Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>, INTERSECTION>: From<(
            &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            &'a [&'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>],
        )> + Iterator<Item = Event>,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Clone + Ord,
    Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            INTERSECTION,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
        >,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>;

    fn intersection(
        self,
        other: &Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    ) -> Self::Output {
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if bounding_box.disjoint_with(&other_bounding_box)
            || bounding_box.touches(&other_bounding_box)
        {
            return vec![];
        }
        let other_coupled_polygons_ids =
            boxes_ids_coupled_with_box(&other_bounding_boxes, &other_bounding_box);
        if other_coupled_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = bounding_box.get_max_x().min(unsafe {
            other_coupled_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let other_coupled_polygons = other_coupled_polygons_ids
            .into_iter()
            .map(|index| &other.polygons[index])
            .collect::<Vec<_>>();
        let mut operation =
            Operation::<Point<_>, INTERSECTION>::from((self, &other_coupled_polygons));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event)
        }
        Polygon::<_>::reduce_events(events, &mut operation)
    }
}
