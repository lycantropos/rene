use std::cmp::Ordering;

use super::event::{is_left_event, Event};

pub(super) struct EventsQueueKey<Endpoint> {
    pub(super) event: Event,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<Event>,
}

impl<Endpoint> EventsQueueKey<Endpoint> {
    pub(super) fn new(event: Event, endpoints: &Vec<Endpoint>, opposites: &Vec<Event>) -> Self {
        Self {
            event,
            endpoints,
            opposites,
        }
    }
}

impl<Endpoint> EventsQueueKey<Endpoint> {
    pub(super) fn endpoints(&self) -> &Vec<Endpoint> {
        unsafe { &(*self.endpoints) }
    }

    pub(super) fn opposites(&self) -> &Vec<Event> {
        unsafe { &(*self.opposites) }
    }
}

impl<Endpoint: PartialEq> PartialEq for EventsQueueKey<Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        self.event == other.event
    }
}

impl<Endpoint: Eq> Eq for EventsQueueKey<Endpoint> {}

impl<Endpoint: Ord> PartialOrd for EventsQueueKey<Endpoint> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_events(
            self.event,
            other.event,
            self.endpoints(),
            self.opposites(),
        ))
    }
}

impl<Endpoint: Ord> Ord for EventsQueueKey<Endpoint> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_events(self.event, other.event, self.endpoints(), self.opposites())
    }
}

fn compare_events<Endpoint: Ord>(
    first_event: Event,
    second_event: Event,
    endpoints: &Vec<Endpoint>,
    opposites: &Vec<Event>,
) -> Ordering {
    match endpoints[first_event].cmp(&endpoints[second_event]) {
        Ordering::Equal => {
            if is_left_event(first_event) != is_left_event(second_event) {
                if is_left_event(first_event) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                endpoints[opposites[first_event]].cmp(&endpoints[opposites[second_event]])
            }
        }
        value => value,
    }
}
