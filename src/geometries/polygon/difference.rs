use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::clipping::{Event, Operation, ReduceEvents, DIFFERENCE};
use crate::geometries::{Empty, Point};
use crate::operations::do_boxes_have_no_common_area;
use crate::relatable::Relatable;
use crate::traits::{Difference, Elemental};

use super::types::Polygon;

impl<Scalar> Difference<Empty> for Polygon<Scalar> {
    type Output = Self;

    fn difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Empty> for Polygon<Scalar> {
    type Output = Self;

    fn difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference<&Empty> for &Polygon<Scalar>
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Digit, const SHIFT: usize> Difference for &Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Self: ReduceEvents<
        Point<Fraction<BigInt<Digit, SHIFT>>>,
        DIFFERENCE,
        Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
    >,
    for<'a> &'a Box<Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> Operation<Point<Fraction<BigInt<Digit, SHIFT>>>, DIFFERENCE>: From<(
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
            &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>,
        )> + Iterator<Item = Event>,
    Fraction<BigInt<Digit, SHIFT>>: PartialOrd,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Bounded<Fraction<BigInt<Digit, SHIFT>>> + Clone,
{
    type Output = Vec<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![self.clone()];
        }
        let max_x = bounding_box.get_max_x();
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((self, other));
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
        Self::reduce_events(events, &mut operation)
    }
}
