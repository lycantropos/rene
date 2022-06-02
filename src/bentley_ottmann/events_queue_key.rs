use std::cmp::Ordering;

use rithm::traits::Parity;

pub(super) struct EventsQueueKey<Endpoint> {
    pub(super) event_index: usize,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<usize>,
}

impl<Endpoint> EventsQueueKey<Endpoint> {
    pub(super) fn new(
        event_index: usize,
        endpoints: &Vec<Endpoint>,
        opposites: &Vec<usize>,
    ) -> Self {
        Self {
            event_index,
            endpoints,
            opposites,
        }
    }
}

impl<Endpoint> EventsQueueKey<Endpoint> {
    pub(super) fn endpoints(&self) -> &Vec<Endpoint> {
        unsafe { &(*self.endpoints) }
    }

    pub(super) fn opposites(&self) -> &Vec<usize> {
        unsafe { &(*self.opposites) }
    }
}

impl<Endpoint: PartialEq> PartialEq for EventsQueueKey<Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        self.event_index == other.event_index
    }
}

impl<Endpoint: Eq> Eq for EventsQueueKey<Endpoint> {}

impl<Endpoint: Ord> PartialOrd for EventsQueueKey<Endpoint> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_events(
            self.event_index,
            other.event_index,
            self.endpoints(),
            self.opposites(),
        ))
    }
}

impl<Endpoint: Ord> Ord for EventsQueueKey<Endpoint> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_events(
            self.event_index,
            other.event_index,
            self.endpoints(),
            self.opposites(),
        )
    }
}

fn compare_events<Endpoint: Ord>(
    left_event_index: usize,
    right_event_index: usize,
    endpoints: &Vec<Endpoint>,
    opposites: &Vec<usize>,
) -> Ordering {
    match endpoints[left_event_index].cmp(&endpoints[right_event_index]) {
        Ordering::Equal => {
            if left_event_index.is_even() != right_event_index.is_even() {
                if left_event_index.is_even() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else {
                endpoints[opposites[left_event_index]].cmp(&endpoints[opposites[right_event_index]])
            }
        }
        value => value,
    }
}
