use std::cmp::Ordering;

use super::event::{is_event_left, Event};

pub(super) struct EventsQueueKey<Point> {
    pub(super) event: Event,
    endpoints: *const Vec<Point>,
    opposites: *const Vec<Event>,
}

impl<Point> EventsQueueKey<Point> {
    pub(super) fn new(
        event: Event,
        endpoints: &Vec<Point>,
        opposites: &Vec<Event>,
    ) -> Self {
        Self {
            event,
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

impl<Point: Ord> PartialOrd for EventsQueueKey<Point> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Point: Ord> Ord for EventsQueueKey<Point> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_events(
            self.event,
            other.event,
            self.get_endpoints(),
            self.get_opposites(),
        )
    }
}

fn compare_events<Point: Ord>(
    first_event: Event,
    second_event: Event,
    endpoints: &[Point],
    opposites: &[Event],
) -> Ordering {
    match endpoints[first_event].cmp(&endpoints[second_event]) {
        Ordering::Equal => {
            if is_event_left(first_event) == is_event_left(second_event) {
                endpoints[opposites[first_event]]
                    .cmp(&endpoints[opposites[second_event]])
            } else if is_event_left(first_event) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        value => value,
    }
}
