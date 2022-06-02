use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::traits::Point;

use super::event::Event;
use super::sweep_line_key::SweepLineKey;

pub(super) struct SweepLine<Scalar, Endpoint> {
    data: BTreeSet<SweepLineKey<Scalar, Endpoint>>,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<Event>,
}

impl<Scalar, Endpoint> SweepLine<Scalar, Endpoint> {
    pub(super) fn new(endpoints: &Vec<Endpoint>, opposites: &Vec<Event>) -> Self {
        Self {
            data: BTreeSet::<SweepLineKey<Scalar, Endpoint>>::new(),
            endpoints,
            opposites,
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
        self.data
            .insert(SweepLineKey::new(event, self.endpoints(), self.opposites()))
    }

    pub(super) fn remove(&mut self, event: Event) -> bool {
        self.data.remove(&SweepLineKey::new(
            event,
            self.endpoints(),
            self.opposites(),
        ))
    }

    pub(super) fn above(&self, event: Event) -> Option<Event> {
        self.data
            .range((
                Excluded(&SweepLineKey::new(
                    event,
                    self.endpoints(),
                    self.opposites(),
                )),
                Unbounded,
            ))
            .next()
            .map(|key| key.event)
    }

    pub(super) fn below(&self, event: Event) -> Option<Event> {
        self.data
            .range((
                Unbounded,
                Excluded(&SweepLineKey::new(
                    event,
                    self.endpoints(),
                    self.opposites(),
                )),
            ))
            .last()
            .map(|key| key.event)
    }

    pub(super) fn find(&self, event: Event) -> Option<Event> {
        self.data
            .get(&SweepLineKey::new(
                event,
                self.endpoints(),
                self.opposites(),
            ))
            .map(|key| key.event)
    }
}
