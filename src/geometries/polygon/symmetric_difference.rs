use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, SYMMETRIC_DIFFERENCE};
use crate::geometries::{Empty, Point};
use crate::relatable::Relatable;
use crate::traits::{Elemental, SymmetricDifference};

use super::types::Polygon;

impl<Scalar> SymmetricDifference<Empty> for Polygon<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<&Empty> for Polygon<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference<&Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> SymmetricDifference
    for &Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    for<'a> &'a Box<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>, SYMMETRIC_DIFFERENCE>:
        From<(
                &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
                &'a Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            )> + Iterator<Item = Event>,
    Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
        + Clone
        + ReduceEvents<
            Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
            SYMMETRIC_DIFFERENCE,
            Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
        >,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if bounding_box.disjoint_with(&other_bounding_box)
            || bounding_box.touches(&other_bounding_box)
        {
            return vec![self.clone(), other.clone()];
        }
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            events.push(event)
        }
        Polygon::<_>::reduce_events(events, &mut operation)
    }
}
