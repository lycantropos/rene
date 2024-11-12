from collections.abc import Iterable, Sequence
from itertools import combinations
from typing import Generic

from typing_extensions import Self

from rene import Relation, hints
from rene._hints import Orienteer, SegmentsIntersector

from .events_registry import EventsRegistry


class Intersection(Generic[hints.Scalar]):
    @property
    def end(self, /) -> hints.Point[hints.Scalar]:
        return self._end

    @property
    def first_segment_id(self, /) -> int:
        return self._first_segment_id

    @property
    def relation(self, /) -> Relation:
        return self._relation

    @property
    def second_segment_id(self, /) -> int:
        return self._second_segment_id

    @property
    def start(self, /) -> hints.Point[hints.Scalar]:
        return self._start

    _end: hints.Point[hints.Scalar]
    _first_segment_id: int
    _relation: Relation
    _second_segment_id: int
    _start: hints.Point[hints.Scalar]

    __slots__ = (
        '_end',
        '_first_segment_id',
        '_relation',
        '_second_segment_id',
        '_start',
    )

    def __new__(
        cls,
        first_segment_id: int,
        second_segment_id: int,
        relation: Relation,
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        /,
    ) -> Self:
        self = super().__new__(cls)
        (
            self._end,
            self._first_segment_id,
            self._relation,
            self._second_segment_id,
            self._start,
        ) = (end, first_segment_id, relation, second_segment_id, start)
        return self


def sweep(
    segments: Sequence[hints.Segment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Iterable[Intersection[hints.Scalar]]:
    events_registry = EventsRegistry.from_segments(
        segments, orienteer, segments_intersector, unique=False
    )
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
    point: hints.Point[hints.Scalar],
    events_registry: EventsRegistry[hints.Scalar],
    /,
) -> Iterable[Intersection[hints.Scalar]]:
    for first_segment_id, second_segment_id in combinations(segments_ids, 2):
        first_start = events_registry.to_segment_start(first_segment_id)
        first_end = events_registry.to_segment_end(first_segment_id)
        second_start = events_registry.to_segment_start(second_segment_id)
        second_end = events_registry.to_segment_end(second_segment_id)
        if first_segment_id == second_segment_id:
            start, end = first_start, first_end
            relation = Relation.EQUAL
        elif not events_registry.are_collinear(
            first_segment_id, second_segment_id
        ):
            if (
                first_start == point
                or first_end == point
                or second_start == point
                or second_end == point
            ):
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
        yield Intersection(
            first_segment_id, second_segment_id, relation, start, end
        )
