use std::cmp::Ordering;

use super::event::{is_left_event, Event};
use crate::operations::Orient;
use crate::oriented::Orientation;

pub(super) struct EventsQueueKey<Point> {
    pub(super) event: Event,
    is_from_first_operand: bool,
    endpoints: *const Vec<Point>,
    opposites: *const Vec<Event>,
}

impl<Point> EventsQueueKey<Point> {
    pub(super) fn new(
        event: Event,
        is_from_first_operand: bool,
        endpoints: &Vec<Point>,
        opposites: &Vec<Event>,
    ) -> Self {
        Self {
            event,
            is_from_first_operand,
            endpoints,
            opposites,
        }
    }
}

impl<Point> EventsQueueKey<Point> {
    fn get_endpoints(&self) -> &[Point] {
        unsafe { &(*self.endpoints) }
    }

    fn get_opposites(&self) -> &[Event] {
        unsafe { &(*self.opposites) }
    }
}

impl<Point: PartialEq> PartialEq for EventsQueueKey<Point> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event
    }
}

impl<Point: Eq> Eq for EventsQueueKey<Point> {}

impl<Point: Ord + Orient> PartialOrd for EventsQueueKey<Point> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_events(
            self.event,
            other.event,
            self.is_from_first_operand,
            other.is_from_first_operand,
            self.get_endpoints(),
            self.get_opposites(),
        ))
    }
}

impl<Point: Ord + Orient> Ord for EventsQueueKey<Point> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_events(
            self.event,
            other.event,
            self.is_from_first_operand,
            other.is_from_first_operand,
            self.get_endpoints(),
            self.get_opposites(),
        )
    }
}

fn compare_events<Point: Ord + Orient>(
    first_event: Event,
    second_event: Event,
    is_first_event_from_first_operand: bool,
    is_second_event_from_first_operand: bool,
    endpoints: &[Point],
    opposites: &[Event],
) -> Ordering {
    match endpoints[first_event].cmp(&endpoints[second_event]) {
        Ordering::Equal => {
            if is_left_event(first_event) == is_left_event(second_event) {
                match endpoints[first_event].orient(
                    &endpoints[opposites[first_event]],
                    &endpoints[opposites[second_event]],
                ) {
                    Orientation::Clockwise => {
                        if is_left_event(first_event) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    }
                    Orientation::Collinear => {
                        debug_assert_ne!(
                            is_first_event_from_first_operand,
                            is_second_event_from_first_operand
                        );
                        if is_first_event_from_first_operand {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    }
                    Orientation::Counterclockwise => {
                        if is_left_event(first_event) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                }
            } else if is_left_event(first_event) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        value => value,
    }
}
