use core::convert::From;
use std::cmp::Ordering;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::iteration::PairwiseCombinations;
use crate::operations::relate_segments;
use crate::relatable::Relation;
use crate::traits::{Point, Segment};

use super::event::Event;
use super::events_registry::EventsRegistry;
use super::traits::EventsQueue;

pub(super) struct Sweep<Scalar, Endpoint> {
    segments_ids_pairs: PairwiseCombinations<usize>,
    start_event: Option<usize>,
    events_registry: EventsRegistry<Scalar, Endpoint, false>,
}

impl<
        Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
        Segment: self::Segment<Scalar, Point = Endpoint>,
    > From<&[Segment]> for Sweep<Scalar, Endpoint>
{
    fn from(segments: &[Segment]) -> Self {
        let mut events_registry = EventsRegistry::from(segments);
        let start_event = events_registry.next();
        Self {
            events_registry,
            start_event,
            segments_ids_pairs: PairwiseCombinations::default(),
        }
    }
}

pub(super) struct Intersection {
    pub(super) first_segment_id: usize,
    pub(super) second_segment_id: usize,
    pub(super) relation: Relation,
}

impl<
        Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    > Iterator for Sweep<Scalar, Endpoint>
{
    type Item = Intersection;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first_segment_id, second_segment_id)) = self.segments_ids_pairs.next() {
            let first_start = self.events_registry.get_segment_start(first_segment_id);
            let first_end = self.events_registry.get_segment_end(first_segment_id);
            let second_start = self.events_registry.get_segment_start(second_segment_id);
            let second_end = self.events_registry.get_segment_end(second_segment_id);
            let relation = if first_segment_id == second_segment_id {
                Relation::Equal
            } else if !self
                .events_registry
                .are_collinear(first_segment_id, second_segment_id)
            {
                if first_start == second_start
                    || first_start == second_end
                    || first_end == second_start
                    || first_end == second_end
                {
                    Relation::Touch
                } else {
                    Relation::Cross
                }
            } else if first_start.max(second_start).eq(first_end.min(second_end)) {
                Relation::Touch
            } else {
                match first_start.cmp(second_start) {
                    Ordering::Equal => match first_end.cmp(second_end) {
                        Ordering::Equal => Relation::Equal,
                        Ordering::Greater => Relation::Composite,
                        Ordering::Less => Relation::Component,
                    },
                    Ordering::Greater => match first_end.cmp(second_end) {
                        Ordering::Greater => Relation::Overlap,
                        _ => Relation::Component,
                    },
                    Ordering::Less => match first_end.cmp(second_end) {
                        Ordering::Less => Relation::Overlap,
                        _ => Relation::Composite,
                    },
                }
            };
            debug_assert_eq!(
                relation,
                relate_segments(first_start, first_end, second_start, second_end)
            );
            Some(Intersection {
                first_segment_id,
                second_segment_id,
                relation,
            })
        } else if let Some(start_event) = self.start_event {
            self.populate_segments_ids_pairs(start_event);
            self.next()
        } else if let Some(start_event) = self.events_registry.pop() {
            self.start_event = Some(start_event);
            self.next()
        } else {
            None
        }
    }
}

impl<
        Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
        Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    > Sweep<Scalar, Endpoint>
{
    fn populate_segments_ids_pairs(&mut self, start_event: Event) {
        let mut segments_ids_containing_start =
            Vec::from([self.events_registry.get_event_segment_id(start_event)]);
        while let Some(event) = self.events_registry.next() {
            if self
                .events_registry
                .get_event_start(start_event)
                .ne(self.events_registry.get_event_start(event))
            {
                self.segments_ids_pairs = PairwiseCombinations::from(segments_ids_containing_start);
                self.start_event = Some(event);
                return;
            }
            segments_ids_containing_start.push(self.events_registry.get_event_segment_id(event));
        }
        self.segments_ids_pairs = PairwiseCombinations::from(segments_ids_containing_start);
        self.start_event = None;
    }
}
