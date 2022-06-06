use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::traits::Point;

use super::event::Event;
use super::events_queue::EventsQueue;
use super::sweep_line_key::SweepLineKey;

pub(super) struct SweepLine<Scalar, Endpoint> {
    data: BTreeSet<SweepLineKey<Scalar, Endpoint>>,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<Event>,
}

impl<Scalar, Endpoint> From<&EventsQueue<Scalar, Endpoint>> for SweepLine<Scalar, Endpoint> {
    fn from(events_queue: &EventsQueue<Scalar, Endpoint>) -> Self {
        Self {
            data: BTreeSet::<SweepLineKey<Scalar, Endpoint>>::new(),
            endpoints: events_queue.get_endpoints(),
            opposites: events_queue.get_opposites(),
        }
    }
}

impl<Scalar, Endpoint> SweepLine<Scalar, Endpoint> {
    fn endpoints(&self) -> &Vec<Endpoint> {
        unsafe { &*self.endpoints }
    }

    fn opposites(&self) -> &Vec<Event> {
        unsafe { &*self.opposites }
    }
}

impl<
        Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + Eq + Point<Scalar>,
    > SweepLine<Scalar, Endpoint>
{
    pub(super) fn insert(&mut self, event: Event) -> bool {
        self.data.insert(self.to_key(event))
    }

    pub(super) fn remove(&mut self, event: Event) -> bool {
        self.data.remove(&self.to_key(event))
    }

    pub(super) fn above(&self, event: Event) -> Option<Event> {
        self.data
            .range((Excluded(&self.to_key(event)), Unbounded))
            .next()
            .map(|key| key.event)
    }

    pub(super) fn below(&self, event: Event) -> Option<Event> {
        self.data
            .range((Unbounded, Excluded(&self.to_key(event))))
            .last()
            .map(|key| key.event)
    }

    pub(super) fn find(&self, event: Event) -> Option<Event> {
        self.data.get(&self.to_key(event)).map(|key| key.event)
    }

    fn to_key(&self, event: Event) -> SweepLineKey<Scalar, Endpoint> {
        SweepLineKey::new(event, self.endpoints(), self.opposites())
    }
}
