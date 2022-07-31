use std::cmp::Ordering;

use crate::operations::Orient;
use crate::oriented::Orientation;
use crate::traits::Point;

use super::event::Event;

pub(super) struct SweepLineKey<Endpoint> {
    pub(super) event: Event,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<Event>,
}

impl<Endpoint> SweepLineKey<Endpoint> {
    pub(super) fn new(event: Event, endpoints: &Vec<Endpoint>, opposites: &Vec<Event>) -> Self {
        Self {
            event,
            endpoints,
            opposites,
        }
    }
}

impl<Endpoint> SweepLineKey<Endpoint> {
    fn get_endpoints(&self) -> &[Endpoint] {
        unsafe { &(*self.endpoints) }
    }

    fn get_opposites(&self) -> &[Event] {
        unsafe { &(*self.opposites) }
    }
}

impl<Endpoint: PartialEq> PartialEq for SweepLineKey<Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event
    }
}

impl<Endpoint: Eq> Eq for SweepLineKey<Endpoint> {}

impl<Scalar: Ord, Endpoint: Orient + Point<Coordinate = Scalar>> PartialOrd
    for SweepLineKey<Endpoint>
where
    Self: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_sweep_line_keys(
            self.event,
            other.event,
            self.get_endpoints(),
            self.get_opposites(),
        ))
    }
}

impl<Scalar: Ord, Endpoint: Orient + Point<Coordinate = Scalar>> Ord for SweepLineKey<Endpoint>
where
    Self: Eq + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        compare_sweep_line_keys(
            self.event,
            other.event,
            self.get_endpoints(),
            self.get_opposites(),
        )
    }
}

fn compare_sweep_line_keys<Scalar: Ord, Endpoint: Orient + Point<Coordinate = Scalar>>(
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

fn compare_segments_position<Scalar: Ord, Endpoint: Orient + Point<Coordinate = Scalar>>(
    first_start: &Endpoint,
    first_end: &Endpoint,
    second_start: &Endpoint,
    second_end: &Endpoint,
) -> Ordering {
    let other_start_orientation = first_start.orient(first_end, second_start);
    let other_end_orientation = first_start.orient(first_end, second_end);
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
        let start_orientation = second_start.orient(second_end, first_start);
        let end_orientation = second_start.orient(second_end, first_end);
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
