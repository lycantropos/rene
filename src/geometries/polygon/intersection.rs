use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, INTERSECTION};
use crate::geometries::{Empty, Point};
use crate::operations::do_boxes_have_no_common_area;
use crate::relatable::Relatable;
use crate::traits::{Elemental, Intersection};

use super::types::Polygon;

impl<Scalar> Intersection<Empty> for Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Digit, const SHIFT: usize> Intersection for &Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: Bounded<Fraction<BigInt<Digit, SHIFT>>>
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SHIFT>>>,
            INTERSECTION,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        >,
    Fraction<BigInt<Digit, SHIFT>>: Ord,
    Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, INTERSECTION>: Iterator<Item = Event>
        + for<'a> From<(
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
        )>,
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> &'a Point<Fraction<BigInt<Digit, SHIFT>>>:
        Elemental<Coordinate = &'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![];
        }
        let min_max_x = bounding_box.get_max_x().min(other_bounding_box.get_max_x());
        let mut operation = Operation::<Point<_>, INTERSECTION>::from((self, other));
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
