from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, TYPE_CHECKING

from dendroid import red_black
from prioq.base import PriorityQueue
from typing_extensions import Self

from rene import Orientation, hints
from rene._utils import is_even, to_sorted_pair

from .event import Event, is_event_left, is_event_right, left_event_to_position
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey

if TYPE_CHECKING:
    from collections.abc import Iterable, Iterator

    from dendroid.hints import KeyedSet

    from rene._hints import Orienteer, SegmentsIntersector


class Operation(ABC, Generic[hints.Scalar]):
    @classmethod
    def from_segments_iterables(
        cls,
        first: Iterable[hints.Segment[hints.Scalar]],
        second: Iterable[hints.Segment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /,
    ) -> Self:
        endpoints: list[hints.Point[hints.Scalar]] = []
        _populate_with_segments(first, endpoints)
        first_segments_count = len(endpoints) >> 1
        _populate_with_segments(second, endpoints)
        second_segments_count = (len(endpoints) >> 1) - first_segments_count
        return cls(
            first_segments_count,
            second_segments_count,
            endpoints,
            orienteer,
            segments_intersector,
        )

    @abstractmethod
    def reduce_events(
        self,
        events: list[Event],
        segment_cls: type[hints.Segment[hints.Scalar]],
        /,
    ) -> list[hints.Segment[hints.Scalar]]:
        pass

    def to_event_end(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.to_event_start(self.to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.endpoints[event]

    def to_opposite_event(self, event: Event, /) -> Event:
        return self._opposites[event]

    _sweep_line_data: KeyedSet[SweepLineKey[hints.Scalar], Event]

    __slots__ = (
        'first_segments_count',
        'second_segments_count',
        'endpoints',
        '_events_queue_data',
        '_opposites',
        '_orienteer',
        '_segments_ids',
        '_segments_intersector',
        '_sweep_line_data',
    )

    def __init__(
        self,
        first_segments_count: int,
        second_segments_count: int,
        endpoints: list[hints.Point[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /,
    ) -> None:
        (
            self.endpoints,
            self.first_segments_count,
            self.second_segments_count,
            self._orienteer,
            self._segments_intersector,
        ) = (
            endpoints,
            first_segments_count,
            second_segments_count,
            orienteer,
            segments_intersector,
        )
        segments_count = first_segments_count + second_segments_count
        initial_events_count = 2 * segments_count
        self._opposites = [
            Event(((index >> 1) << 1) + is_even(index))
            for index in range(initial_events_count)
        ]
        self._segments_ids = list(range(segments_count))
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[hints.Scalar], Event
        ] = PriorityQueue(
            *map(Event, range(initial_events_count)),
            key=lambda event: EventsQueueKey(
                event,
                self._is_event_from_first_operand(event),
                self.endpoints,
                self._opposites,
                self._orienteer,
            ),
        )
        self._sweep_line_data = red_black.set_(key=self._to_sweep_line_key)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self) -> Iterator[Event]:
        while self:
            event = self._pop()
            if is_event_right(event):
                opposite_event = self.to_opposite_event(event)
                assert is_event_left(opposite_event)
                equal_segment_event = self._find(opposite_event)
                if equal_segment_event is not None:
                    above_event, below_event = (
                        self._above(equal_segment_event),
                        self._below(equal_segment_event),
                    )
                    self._remove(equal_segment_event)
                    if below_event is not None and above_event is not None:
                        self._detect_intersection(below_event, above_event)
            elif self._find(event) is None:
                self._add(event)
                above_event, below_event = (
                    self._above(event),
                    self._below(event),
                )
                if above_event is not None:
                    self._detect_intersection(event, above_event)
                if below_event is not None:
                    self._detect_intersection(below_event, event)
            yield event

    def _above(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.next(event)
        except ValueError:
            return None

    def _add(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.add(event)

    def _below(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.prev(event)
        except ValueError:
            return None

    def _detect_intersection(
        self, below_event: Event, event: Event, /
    ) -> None:
        event_start = self.to_event_start(event)
        event_end = self.to_event_end(event)
        below_event_start = self.to_event_start(below_event)
        below_event_end = self.to_event_end(below_event)
        event_start_orientation = self._orienteer(
            below_event_end, below_event_start, event_start
        )
        event_end_orientation = self._orienteer(
            below_event_end, below_event_start, event_end
        )
        if event_start_orientation is event_end_orientation:
            if event_start_orientation is Orientation.COLLINEAR:
                assert self._is_left_event_from_first_operand(
                    below_event
                ) is not self._is_left_event_from_first_operand(event)
                if event_start == below_event_start:
                    if event_end != below_event_end:
                        max_end_event, min_end_event = (
                            (below_event, event)
                            if event_end < below_event_end
                            else (event, below_event)
                        )
                        min_end = self.to_event_end(min_end_event)
                        min_end_start_event, min_end_max_end_event = (
                            self._divide(max_end_event, min_end)
                        )
                        self._push(min_end_start_event)
                        self._push(min_end_max_end_event)
                elif event_end == below_event_end:
                    max_start_event, min_start_event = (
                        (below_event, event)
                        if event_start < below_event_start
                        else (event, below_event)
                    )
                    max_start = self.to_event_start(max_start_event)
                    (max_start_to_min_start_event, max_start_to_end_event) = (
                        self._divide(min_start_event, max_start)
                    )
                    self._push(max_start_to_min_start_event)
                    self._push(max_start_to_end_event)
                elif below_event_start < event_start < below_event_end:
                    if event_end < below_event_end:
                        self._divide_event_by_mid_segment_event_endpoints(
                            below_event, event_start, event_end
                        )
                    else:
                        max_start, min_end = event_start, below_event_end
                        self._divide_overlapping_events(
                            below_event, event, max_start, min_end
                        )
                elif event_start < below_event_start < event_end:
                    if below_event_end < event_end:
                        self._divide_event_by_mid_segment_event_endpoints(
                            event, below_event_start, below_event_end
                        )
                    else:
                        max_start, min_end = below_event_start, event_end
                        self._divide_overlapping_events(
                            event, below_event, max_start, min_end
                        )
        elif event_start_orientation is Orientation.COLLINEAR:
            if below_event_start < event_start < below_event_end:
                point = event_start
                self._divide_event_by_midpoint(below_event, point)
        elif event_end_orientation is Orientation.COLLINEAR:
            if below_event_start < event_end < below_event_end:
                point = event_end
                self._divide_event_by_midpoint(below_event, point)
        else:
            below_event_start_orientation = self._orienteer(
                event_start, event_end, below_event_start
            )
            below_event_end_orientation = self._orienteer(
                event_start, event_end, below_event_end
            )
            if below_event_start_orientation is Orientation.COLLINEAR:
                assert below_event_end_orientation is not Orientation.COLLINEAR
                if event_start < below_event_start < event_end:
                    point = below_event_start
                    self._divide_event_by_midpoint(event, point)
            elif below_event_end_orientation is Orientation.COLLINEAR:
                if event_start < below_event_end < event_end:
                    point = below_event_end
                    self._divide_event_by_midpoint(event, point)
            elif (
                below_event_start_orientation
                is not below_event_end_orientation
            ):
                cross_point = self._segments_intersector(
                    event_start, event_end, below_event_start, below_event_end
                )
                assert event_start < cross_point < event_end
                assert below_event_start < cross_point < below_event_end
                self._divide_event_by_midpoint(below_event, cross_point)
                self._divide_event_by_midpoint(event, cross_point)

    def _divide(
        self, event: Event, mid_point: hints.Point[hints.Scalar], /
    ) -> tuple[Event, Event]:
        assert is_event_left(event)
        opposite_event = self.to_opposite_event(event)
        mid_point_to_event_end_event: Event = Event(len(self.endpoints))
        self._segments_ids.append(self._left_event_to_segment_id(event))
        self.endpoints.append(mid_point)
        self._opposites.append(opposite_event)
        self._opposites[opposite_event] = mid_point_to_event_end_event
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        assert self._is_left_event_from_first_operand(
            event
        ) is self._is_event_from_first_operand(mid_point_to_event_start_event)
        assert self._is_left_event_from_first_operand(
            event
        ) is self._is_left_event_from_first_operand(
            mid_point_to_event_end_event
        )
        return mid_point_to_event_start_event, mid_point_to_event_end_event

    def _divide_event_by_mid_segment_event_endpoints(
        self,
        event: Event,
        mid_segment_event_start: hints.Point[hints.Scalar],
        mid_segment_event_end: hints.Point[hints.Scalar],
        /,
    ) -> None:
        self._divide_event_by_midpoint(event, mid_segment_event_end)
        self._divide_event_by_midpoint(event, mid_segment_event_start)

    def _divide_event_by_midpoint(
        self, event: Event, point: hints.Point[hints.Scalar], /
    ) -> None:
        point_to_event_start_event, point_to_event_end_event = self._divide(
            event, point
        )
        self._push(point_to_event_start_event)
        self._push(point_to_event_end_event)

    def _divide_overlapping_events(
        self,
        min_start_event: Event,
        max_start_event: Event,
        max_start: hints.Point[hints.Scalar],
        min_end: hints.Point[hints.Scalar],
        /,
    ) -> None:
        self._divide_event_by_midpoint(max_start_event, min_end)
        self._divide_event_by_midpoint(min_start_event, max_start)

    def _find(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        candidate = self._sweep_line_data.tree.find(
            self._to_sweep_line_key(event)
        )
        return None if candidate is red_black.NIL else candidate.value

    def _is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
            self._to_left_event(event)
        )

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
        return (
            self._left_event_to_segment_id(event) < self.first_segments_count
        )

    def _left_event_to_segment_id(self, event: Event, /) -> int:
        return self._segments_ids[left_event_to_position(event)]

    def _pop(self) -> Event:
        return self._events_queue_data.pop()

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.remove(event)

    def _to_event_endpoints(
        self, event: Event, /
    ) -> tuple[hints.Point[hints.Scalar], hints.Point[hints.Scalar]]:
        return self.to_event_start(event), self.to_event_end(event)

    def _to_left_event(self, event: Event, /) -> Event:
        return event if is_event_left(event) else self.to_opposite_event(event)

    def _to_sweep_line_key(
        self, event: Event, /
    ) -> SweepLineKey[hints.Scalar]:
        return SweepLineKey(
            event,
            self._is_left_event_from_first_operand(event),
            self.endpoints,
            self._opposites,
            self._orienteer,
        )


def _populate_with_segments(
    segments: Iterable[hints.Segment[hints.Scalar]],
    endpoints: list[hints.Point[hints.Scalar]],
    /,
) -> None:
    for segment in segments:
        start, end = to_sorted_pair(segment.start, segment.end)
        endpoints.append(start)
        endpoints.append(end)
