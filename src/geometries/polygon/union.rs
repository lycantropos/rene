use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, UNION};
use crate::geometries::{Empty, Point};
use crate::operations::do_boxes_have_no_common_continuum;
use crate::relatable::Relatable;
use crate::traits::{Elemental, Union};

use super::types::Polygon;

impl<Scalar> Union<Empty> for Polygon<Scalar> {
    type Output = Self;

    fn union(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<&Empty> for Polygon<Scalar> {
    type Output = Self;

    fn union(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn union(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Union<&Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn union(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Digit, const SHIFT: usize> Union for &Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: ReduceEvents<
        Point<Fraction<BigInt<Digit, SHIFT>>>,
        UNION,
        Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
    >,
    Fraction<BigInt<Digit, SHIFT>>: PartialEq,
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>> + Clone,
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, UNION>: From<(
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
        )> + Iterator<Item = Event>,
    for<'a> &'a Point<Fraction<BigInt<Digit, SHIFT>>>:
        Elemental<Coordinate = &'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn union(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(&bounding_box, &other_bounding_box) {
            return vec![self.clone(), other.clone()];
        }
        let mut operation = Operation::<Point<_>, UNION>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        for event in operation.by_ref() {
            events.push(event);
        }
        Self::reduce_events(events, &mut operation)
    }
}
