use std::cmp::Ordering;

use crate::operations::Orient;
use crate::oriented::Orientation;
use crate::traits::Elemental;

use super::event::Event;

pub(super) struct SweepLineKey<Point> {
    pub(super) event: Event,
    endpoints: *const Vec<Point>,
    opposites: *const Vec<Event>,
}

impl<Point> SweepLineKey<Point> {
    pub(super) fn new(event: Event, endpoints: &Vec<Point>, opposites: &Vec<Event>) -> Self {
        Self {
            event,
            endpoints,
            opposites,
        }
    }
}

impl<Point> SweepLineKey<Point> {
    fn get_endpoints(&self) -> &[Point] {
        unsafe { &(*self.endpoints) }
    }

    fn get_opposites(&self) -> &[Event] {
        unsafe { &(*self.opposites) }
    }
}

impl<Point: PartialEq> PartialEq for SweepLineKey<Point> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event
    }
}

impl<Point: Eq> Eq for SweepLineKey<Point> {}

impl<Scalar: Ord, Point: Elemental<Coordinate = Scalar>> PartialOrd for SweepLineKey<Point>
where
    Self: PartialEq,
    for<'a> &'a Point: Orient,
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

impl<Scalar: Ord, Point: Elemental<Coordinate = Scalar>> Ord for SweepLineKey<Point>
where
    Self: Eq + PartialOrd,
    for<'a> &'a Point: Orient,
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

fn compare_sweep_line_keys<Scalar: Ord, Point: Elemental<Coordinate = Scalar>>(
    left_event: Event,
    right_event: Event,
    endpoints: &[Point],
    opposites: &[Event],
) -> Ordering
where
    for<'a> &'a Point: Orient,
{
    compare_segments_position(
        &endpoints[left_event],
        &endpoints[opposites[left_event]],
        &endpoints[right_event],
        &endpoints[opposites[right_event]],
    )
}

fn compare_segments_position<Scalar: Ord, Point: Elemental<Coordinate = Scalar>>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Ordering
where
    for<'a> &'a Point: Orient,
{
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
