use std::cmp::Ordering;
use std::marker::PhantomData;

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::operations::orient;
use crate::oriented::Orientation;
use crate::traits::Point;

use super::event::Event;

pub(super) struct SweepLineKey<Scalar, Endpoint> {
    pub(super) event: Event,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<Event>,
    _phantom: PhantomData<fn() -> Scalar>,
}

impl<Scalar, Endpoint> SweepLineKey<Scalar, Endpoint> {
    pub(super) fn new(event: Event, endpoints: &Vec<Endpoint>, opposites: &Vec<Event>) -> Self {
        Self {
            event,
            endpoints,
            opposites,
            _phantom: PhantomData,
        }
    }
}

impl<Scalar, Endpoint> SweepLineKey<Scalar, Endpoint> {
    fn endpoints(&self) -> &[Endpoint] {
        unsafe { &(*self.endpoints) }
    }

    fn opposites(&self) -> &[Event] {
        unsafe { &(*self.opposites) }
    }
}

impl<Scalar, Endpoint: PartialEq> PartialEq for SweepLineKey<Scalar, Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event
    }
}

impl<Scalar, Endpoint: Eq> Eq for SweepLineKey<Scalar, Endpoint> {}

impl<
        Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
        Endpoint: PartialEq + Point<Scalar>,
    > PartialOrd for SweepLineKey<Scalar, Endpoint>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_sweep_line_keys(
            self.event,
            other.event,
            self.endpoints(),
            self.opposites(),
        ))
    }
}

impl<Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed, Endpoint: Eq + Point<Scalar>> Ord
    for SweepLineKey<Scalar, Endpoint>
{
    fn cmp(&self, other: &Self) -> Ordering {
        compare_sweep_line_keys(self.event, other.event, self.endpoints(), self.opposites())
    }
}

fn compare_sweep_line_keys<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
    Endpoint: PartialEq + Point<Scalar>,
>(
    left_event: Event,
    right_event: Event,
    endpoints: &[Endpoint],
    opposites: &[Event],
) -> Ordering {
    compare_segments_position(
        &endpoints[left_event],
        &endpoints[opposites[left_event]],
        &endpoints[right_event],
        &endpoints[opposites[right_event]],
    )
}

fn compare_segments_position<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
    Endpoint: PartialEq + Point<Scalar>,
>(
    first_start: &Endpoint,
    first_end: &Endpoint,
    second_start: &Endpoint,
    second_end: &Endpoint,
) -> Ordering {
    let other_start_orientation = orient(first_start, first_end, second_start);
    let other_end_orientation = orient(first_start, first_end, second_end);
    if other_start_orientation == other_end_orientation {
        match other_start_orientation {
            Orientation::Collinear => match first_start.y().cmp(&second_start.y()) {
                Ordering::Equal => match first_start.x().cmp(&second_start.x()) {
                    Ordering::Equal => match first_end.y().cmp(&second_end.y()) {
                        Ordering::Equal => first_end.x().cmp(&second_end.x()),
                        value => value,
                    },
                    value => value,
                },
                value => value,
            },
            Orientation::Clockwise => Ordering::Greater,
            Orientation::Counterclockwise => Ordering::Less,
        }
    } else {
        let start_orientation = orient(second_start, second_end, first_start);
        let end_orientation = orient(second_start, second_end, first_end);
        if start_orientation == end_orientation {
            match start_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if other_start_orientation == Orientation::Collinear {
            match other_end_orientation {
                Orientation::Counterclockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if start_orientation == Orientation::Collinear {
            match end_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if end_orientation == Orientation::Collinear {
            match start_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else {
            match other_start_orientation {
                Orientation::Counterclockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        }
    }
}
