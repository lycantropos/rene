use crate::bounded::{Bounded, Box};
use crate::clipping::shaped::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, DIFFERENCE};
use crate::geometries::{Empty, Point};
use crate::operations::do_boxes_have_no_common_area;
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
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

impl<Scalar> Difference for &Polygon<Scalar>
where
    Scalar: PartialOrd,
    Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a Polygon<Scalar>, &'a Polygon<Scalar>)>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![self.clone()];
        }
        let max_x = bounding_box.get_max_x();
        let mut operation =
            Operation::<Point<_>, DIFFERENCE>::from((self, other));
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
        operation.reduce_events(events)
    }
}
