from itertools import combinations
from typing import (Iterable,
                    Sequence)

from reprit.base import generate_repr

from rene._rene import Relation
from rene.hints import (Point,
                        Segment)
from .event import segment_id_to_left_event
from .events_registry import EventsRegistry
from .utils import intersect_crossing_segments


class Intersection:
    @property
    def end(self) -> Point:
        return self._end

    @property
    def first_segment_id(self) -> int:
        return self._first_segment_id

    @property
    def relation(self) -> Relation:
        return self._relation

    @property
    def second_segment_id(self) -> int:
        return self._second_segment_id

    @property
    def start(self) -> Point:
        return self._start

    __slots__ = ('_end', '_first_segment_id', '_relation',
                 '_second_segment_id', '_start')

    def __init__(self,
                 first_segment_id: int,
                 second_segment_id: int,
                 relation: Relation,
                 start: Point,
                 end: Point) -> None:
        (
            self._end, self._first_segment_id, self._relation,
            self._second_segment_id, self._start
        ) = end, first_segment_id, relation, second_segment_id, start

    __repr__ = generate_repr(__init__)


def sweep(segments: Sequence[Segment]) -> Iterable[Intersection]:
    events_registry = EventsRegistry.from_segments(segments,
                                                   unique=False)
    events = iter(events_registry)
    event = next(events)
    start = events_registry.to_event_start(event)
    segments_ids_containing_start = [
        events_registry.to_event_segment_id(event)
    ]
    for event in events:
        event_start = events_registry.to_event_start(event)
        if event_start == start:
            segments_ids_containing_start.append(
                    events_registry.to_event_segment_id(event)
            )
        else:
            yield from segments_ids_containing_point_to_intersections(
                    segments_ids_containing_start, start, events_registry
            )
            start = event_start
            segments_ids_containing_start = [
                events_registry.to_event_segment_id(event)
            ]
    yield from segments_ids_containing_point_to_intersections(
            segments_ids_containing_start, start, events_registry
    )


def segments_ids_containing_point_to_intersections(
        segments_ids: Sequence[int],
        point: Point,
        events_registry: EventsRegistry
) -> Iterable[Intersection]:
    for first_segment_id, second_segment_id in combinations(segments_ids, 2):
        first_start = events_registry.to_segment_start(first_segment_id)
        first_end = events_registry.to_segment_end(first_segment_id)
        second_start = events_registry.to_segment_start(second_segment_id)
        second_end = events_registry.to_segment_end(second_segment_id)
        if first_segment_id == second_segment_id:
            start, end = first_start, first_end
            relation = Relation.EQUAL
        elif not events_registry.are_collinear(first_segment_id,
                                               second_segment_id):
            if (first_start == point or first_end == point
                    or second_start == point or second_end == point):
                relation = Relation.TOUCH
            else:
                relation = Relation.CROSS
            start = end = point
        elif max(first_start, second_start) == min(first_end, second_end):
            relation = Relation.TOUCH
            start = end = point
        elif first_start == second_start:
            start = first_start
            if first_end == second_end:
                relation = Relation.EQUAL
                end = first_end
            elif first_end > second_end:
                relation = Relation.COMPOSITE
                end = second_end
            else:
                relation = Relation.COMPONENT
                end = first_end
        elif first_start > second_start:
            start = first_start
            if first_end > second_end:
                relation = Relation.OVERLAP
                end = second_end
            else:
                relation = Relation.COMPONENT
                end = first_end
        elif first_end < second_end:
            relation = Relation.OVERLAP
            start, end = second_start, first_end
        else:
            relation = Relation.COMPOSITE
            start, end = second_start, second_end
        yield Intersection(first_segment_id, second_segment_id, relation,
                           start, end)
