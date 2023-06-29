use core::convert::From;
use std::cmp::Ordering;

use crate::iteration::PairwiseCombinations;
use crate::operations::Orient;
use crate::relatable::Relation;
use crate::relating::segment;

use super::event::Event;
use super::events_registry::EventsRegistry;

pub(crate) struct Sweep<Point> {
    events_registry: EventsRegistry<Point, false>,
    next_start_event: Option<usize>,
    segments_ids_pairs: PairwiseCombinations<usize>,
    start_event: Option<usize>,
}

impl<Point, Input> From<Input> for Sweep<Point>
where
    EventsRegistry<Point, false>: From<Input> + Iterator<Item = Event>,
{
    fn from(input: Input) -> Self {
        let mut events_registry = EventsRegistry::from(input);
        let next_start_event = events_registry.next();
        Self {
            events_registry,
            next_start_event,
            segments_ids_pairs: PairwiseCombinations::default(),
            start_event: None,
        }
    }
}

impl<Point> Sweep<Point> {
    pub(super) fn get_segment_end(&self, segment_id: usize) -> &Point {
        self.events_registry.get_segment_end(segment_id)
    }

    pub(super) fn get_segment_start(&self, segment_id: usize) -> &Point {
        self.events_registry.get_segment_start(segment_id)
    }
}

pub(crate) struct Intersection<Point> {
    pub(super) first_segment_id: usize,
    pub(super) second_segment_id: usize,
    pub(super) relation: Relation,
    pub(super) start: Point,
    pub(super) end: Point,
}

impl<Point: Clone + Ord> Iterator for Sweep<Point>
where
    EventsRegistry<Point, false>: Iterator<Item = Event>,
    for<'a> &'a Point: Orient,
{
    type Item = Intersection<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first_segment_id, second_segment_id)) =
            self.segments_ids_pairs.next()
        {
            let first_start = self.get_segment_start(first_segment_id);
            let first_end = self.get_segment_end(first_segment_id);
            let second_start = self.get_segment_start(second_segment_id);
            let second_end = self.get_segment_end(second_segment_id);
            let (relation, start, end) = if first_segment_id
                == second_segment_id
            {
                (Relation::Equal, first_start, first_end)
            } else if !self
                .events_registry
                .are_collinear(first_segment_id, second_segment_id)
            {
                if let Some(start_event) = self.start_event {
                    let start =
                        self.events_registry.get_event_start(start_event);
                    (
                        if first_start == start
                            || first_end == start
                            || second_start == start
                            || second_end == start
                        {
                            Relation::Touch
                        } else {
                            Relation::Cross
                        },
                        start,
                        start,
                    )
                } else {
                    debug_assert!(first_start == second_start);
                    (Relation::Touch, first_start, first_start)
                }
            } else if first_start
                .max(second_start)
                .eq(first_end.min(second_end))
            {
                let point = first_start.max(second_start);
                (Relation::Touch, point, point)
            } else {
                match first_start.cmp(second_start) {
                    Ordering::Equal => match first_end.cmp(second_end) {
                        Ordering::Equal => {
                            (Relation::Equal, first_start, first_end)
                        }
                        Ordering::Greater => {
                            (Relation::Composite, first_start, second_end)
                        }
                        Ordering::Less => {
                            (Relation::Component, first_start, first_end)
                        }
                    },
                    Ordering::Greater => match first_end.cmp(second_end) {
                        Ordering::Greater => {
                            (Relation::Overlap, first_start, second_end)
                        }
                        _ => (Relation::Component, first_start, first_end),
                    },
                    Ordering::Less => match first_end.cmp(second_end) {
                        Ordering::Less => {
                            (Relation::Overlap, second_start, first_end)
                        }
                        _ => (Relation::Composite, second_start, second_end),
                    },
                }
            };
            debug_assert_eq!(
                relation,
                segment::relate_to_segment(
                    first_start,
                    first_end,
                    second_start,
                    second_end
                )
            );
            Some(Intersection {
                first_segment_id,
                second_segment_id,
                relation,
                start: start.clone(),
                end: end.clone(),
            })
        } else if let Some(next_start_event) = self.next_start_event {
            self.populate_segments_ids_pairs(next_start_event);
            self.next()
        } else {
            None
        }
    }
}

impl<Point: PartialEq> Sweep<Point>
where
    EventsRegistry<Point, false>: Iterator<Item = Event>,
{
    fn populate_segments_ids_pairs(&mut self, start_event: Event) {
        let mut segments_ids_containing_start =
            Vec::from([self.events_registry.to_event_segment_id(start_event)]);
        while let Some(event) = self.events_registry.next() {
            if self
                .events_registry
                .get_event_start(start_event)
                .ne(self.events_registry.get_event_start(event))
            {
                self.segments_ids_pairs =
                    PairwiseCombinations::from(segments_ids_containing_start);
                (self.start_event, self.next_start_event) =
                    (self.next_start_event, Some(event));
                return;
            }
            segments_ids_containing_start
                .push(self.events_registry.to_event_segment_id(event));
        }
        self.segments_ids_pairs =
            PairwiseCombinations::from(segments_ids_containing_start);
        (self.start_event, self.next_start_event) =
            (self.next_start_event, None);
    }
}
