use crate::bounded::{Bounded, Box};
use crate::clipping::shaped::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, SYMMETRIC_DIFFERENCE};
use crate::geometries::{Empty, Point};
use crate::operations::do_boxes_have_no_common_continuum;
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

impl<Scalar> SymmetricDifference for &Polygon<Scalar>
where
    Scalar: PartialEq,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a Polygon<Scalar>, &'a Polygon<Scalar>)>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![self.clone(), other.clone()];
        }
        let mut operation =
            Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        for event in operation.by_ref() {
            events.push(event);
        }
        operation.reduce_events(events)
    }
}
