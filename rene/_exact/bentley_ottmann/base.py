from itertools import combinations
from typing import (Iterable,
                    Sequence)

from reprit.base import generate_repr

from rene._rene import Relation
from rene.hints import Segment
from .events_registry import EventsRegistry


class Intersection:
    @property
    def first_segment_id(self) -> int:
        return self._first_segment_id

    @property
    def relation(self) -> Relation:
        return self._relation

    @property
    def second_segment_id(self) -> int:
        return self._second_segment_id

    __slots__ = '_first_segment_id', '_second_segment_id', '_relation'

    def __init__(self,
                 first_segment_id: int,
                 second_segment_id: int,
                 relation: Relation) -> None:
        self._first_segment_id, self._relation, self._second_segment_id = (
            first_segment_id, relation, second_segment_id
        )

    __repr__ = generate_repr(__init__)


def sweep(segments: Sequence[Segment]) -> Iterable[Intersection]:
    events_registry = EventsRegistry.from_segments(segments,
                                                   unique=False)
    events = iter(events_registry)
    event = next(events)
    events_registry.add(event)
    start = events_registry.get_event_start(event)
    segments_ids_containing_start = [
        events_registry.get_event_segment_id(event)
    ]
    for event in events:
        event_start = events_registry.get_event_start(event)
        if event_start == start:
            segments_ids_containing_start.append(
                    events_registry.get_event_segment_id(event)
            )
        else:
            yield from segments_ids_containing_start_to_intersections(
                    segments_ids_containing_start, events_registry
            )
            start = event_start
            segments_ids_containing_start = [
                events_registry.get_event_segment_id(event)
            ]
    yield from segments_ids_containing_start_to_intersections(
            segments_ids_containing_start, events_registry
    )


def segments_ids_containing_start_to_intersections(
        segments_ids: Sequence[int],
        events_registry: EventsRegistry
) -> Iterable[Intersection]:
    for first_segment_id, second_segment_id in combinations(segments_ids, 2):
        first_start = events_registry.get_segment_start(first_segment_id)
        first_end = events_registry.get_segment_end(first_segment_id)
        second_start = events_registry.get_segment_start(second_segment_id)
        second_end = events_registry.get_segment_end(second_segment_id)
        if first_segment_id == second_segment_id:
            relation = Relation.EQUAL
        elif not events_registry.are_collinear(first_segment_id,
                                               second_segment_id):
            if (first_start == second_start or first_start == second_end
                    or first_end == second_start or first_end == second_end):
                relation = Relation.TOUCH
            else:
                relation = Relation.CROSS
        elif max(first_start, second_start) == min(first_end, second_end):
            relation = Relation.TOUCH
        elif first_start == second_start:
            if first_end == second_end:
                relation = Relation.EQUAL
            elif first_end > second_end:
                relation = Relation.COMPOSITE
            else:
                relation = Relation.COMPONENT
        elif first_start > second_start:
            if first_end > second_end:
                relation = Relation.OVERLAP
            else:
                relation = Relation.COMPONENT
        elif first_end < second_end:
            relation = Relation.OVERLAP
        else:
            relation = Relation.COMPOSITE
        yield Intersection(first_segment_id, second_segment_id, relation)
