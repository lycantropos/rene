use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, INTERSECTION};
use crate::geometries::Point;
use crate::relatable::Relatable;
use crate::traits::{Elemental, Intersection};

use super::types::Polygon;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Intersection
    for &Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>, INTERSECTION>: From<(
            &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
        )> + Iterator<Item = Event>,
    Fraction<BigInt<Digit, SEPARATOR, SHIFT>>: Ord,
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

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if bounding_box.disjoint_with(&other_bounding_box)
            || bounding_box.touches(&other_bounding_box)
        {
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
            events.push(event)
        }
        Polygon::<_>::reduce_events(events, &mut operation)
    }
}
