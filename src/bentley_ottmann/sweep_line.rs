use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::traits::Point;

use super::sweep_line_key::SweepLineKey;

pub(super) struct SweepLine<Scalar, Endpoint> {
    data: BTreeSet<SweepLineKey<Scalar, Endpoint>>,
    endpoints: *const Vec<Endpoint>,
    opposites: *const Vec<usize>,
}

impl<Scalar, Endpoint> SweepLine<Scalar, Endpoint> {
    pub(super) fn new(endpoints: &Vec<Endpoint>, opposites: &Vec<usize>) -> Self {
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

    fn opposites(&self) -> &Vec<usize> {
        unsafe { &*self.opposites }
    }
}

impl<
        Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + Eq + Point<Scalar>,
    > SweepLine<Scalar, Endpoint>
{
    pub(super) fn insert(&mut self, event_index: usize) -> bool {
        self.data.insert(SweepLineKey::new(
            event_index,
            self.endpoints(),
            self.opposites(),
        ))
    }

    pub(super) fn remove(&mut self, event_index: usize) -> bool {
        self.data.remove(&SweepLineKey::new(
            event_index,
            self.endpoints(),
            self.opposites(),
        ))
    }

    pub(super) fn above(&self, event_index: usize) -> Option<usize> {
        self.data
            .range((
                Excluded(&SweepLineKey::new(
                    event_index,
                    self.endpoints(),
                    self.opposites(),
                )),
                Unbounded,
            ))
            .next()
            .map(|key| key.event_index)
    }

    pub(super) fn below(&self, event_index: usize) -> Option<usize> {
        self.data
            .range((
                Unbounded,
                Excluded(&SweepLineKey::new(
                    event_index,
                    self.endpoints(),
                    self.opposites(),
                )),
            ))
            .last()
            .map(|key| key.event_index)
    }

    pub(super) fn find(&self, event_index: usize) -> Option<usize> {
        self.data
            .get(&SweepLineKey::new(
                event_index,
                self.endpoints(),
                self.opposites(),
            ))
            .map(|key| key.event_index)
    }
}
