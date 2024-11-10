from __future__ import annotations

import typing as t
from abc import ABC, abstractmethod

import typing_extensions as te
from dendroid import red_black
from dendroid.hints import KeyedSet
from prioq.base import PriorityQueue

from rene import Orientation, hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import is_even

from .event import Event, is_event_left, is_event_right, left_event_to_position
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey


class Operation(ABC, t.Generic[hints.Scalar]):
    @classmethod
    def from_segments_iterables(
        cls,
        first: t.Iterable[hints.Segment[hints.Scalar]],
        second: t.Iterable[hints.Segment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /,
    ) -> te.Self:
        endpoints: list[hints.Point[hints.Scalar]] = []
        have_interior_to_left: list[bool] = []
        _populate_with_segments(first, endpoints, have_interior_to_left)
        first_segments_count = len(have_interior_to_left)
        _populate_with_segments(second, endpoints, have_interior_to_left)
        second_segments_count = len(have_interior_to_left) - first_segments_count
        return cls(
            first_segments_count,
            second_segments_count,
            endpoints,
            have_interior_to_left,
            orienteer,
            segments_intersector,
        )

    def reduce_events(
        self,
        events: list[Event],
        segment_cls: type[hints.Segment[hints.Scalar]],
        /,
    ) -> list[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(self.to_event_start(event), self.to_event_end(event))
            for event in events
            if self._is_event_from_result(event)
        ]

    def to_event_end(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.endpoints[event]

    _sweep_line_data: KeyedSet[SweepLineKey[hints.Scalar], Event]

    __slots__ = (
        "endpoints",
        "first_segments_count",
        "have_interior_to_left",
        "second_segments_count",
        "_are_from_result",
        "_events_queue_data",
        "_have_overlap",
        "_opposites",
        "_orienteer",
        "_other_have_interior_to_left",
        "_segments_ids",
        "_segments_intersector",
        "_sweep_line_data",
    )

    def __init__(
        self,
        first_segments_count: int,
        second_segments_count: int,
        endpoints: list[hints.Point[hints.Scalar]],
        have_interior_to_left: t.Sequence[bool],
        orienteer: Orienteer[hints.Scalar],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /,
    ) -> None:
        (
            self.endpoints,
            self.first_segments_count,
            self.have_interior_to_left,
            self.second_segments_count,
            self._orienteer,
            self._segments_intersector,
        ) = (
            endpoints,
            first_segments_count,
            have_interior_to_left,
            second_segments_count,
            orienteer,
            segments_intersector,
        )
        segments_count = first_segments_count + second_segments_count
        initial_events_count = 2 * segments_count
        self._are_from_result = [False] * segments_count
        self._opposites = [
            Event(((index >> 1) << 1) + is_even(index))
            for index in range(initial_events_count)
        ]
        self._other_have_interior_to_left = [False] * segments_count
        self._have_overlap = [False] * segments_count
        self._segments_ids = list(range(segments_count))
        self._events_queue_data: PriorityQueue[EventsQueueKey[hints.Scalar], Event] = (
            PriorityQueue(
                *map(Event, range(initial_events_count)),
                key=lambda event: EventsQueueKey(
                    event,
                    self._is_event_from_first_operand(event),
                    self.endpoints,
                    self._opposites,
                    self._orienteer,
                ),
            )
        )
        self._sweep_line_data = red_black.set_(key=self._to_sweep_line_key)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self) -> t.Iterator[Event]:
        while self:
            event = self._pop()
            if is_event_right(event):
                opposite_event = self._to_opposite_event(event)
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
                yield event
            elif self._find(event) is None:
                self._add(event)
                above_event, below_event = (
                    self._above(event),
                    self._below(event),
                )
                self._compute_left_event_fields(event, below_event)
                if above_event is not None and self._detect_intersection(
                    event, above_event
                ):
                    self._compute_left_event_fields(event, below_event)
                    self._compute_left_event_fields(above_event, event)
                if below_event is not None and self._detect_intersection(
                    below_event, event
                ):
                    below_below_event = self._below(below_event)
                    self._compute_left_event_fields(below_event, below_below_event)
                    self._compute_left_event_fields(event, below_event)
                yield event

    @property
    def _events_count(self, /) -> int:
        return len(self.endpoints)

    @abstractmethod
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        pass

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

    def _compute_left_event_fields(
        self, event: Event, below_event: Event | None, /
    ) -> None:
        event_position = left_event_to_position(event)
        if below_event is not None:
            below_event_position = left_event_to_position(below_event)
            self._other_have_interior_to_left[event_position] = (
                self._other_have_interior_to_left[below_event_position]
                if (
                    self._is_left_event_from_first_operand(event)
                    is self._is_left_event_from_first_operand(below_event)
                )
                else self.have_interior_to_left[
                    self._left_event_to_segment_id(below_event)
                ]
            )
        self._are_from_result[event_position] = self._detect_if_left_event_from_result(
            event
        )

    def _detect_intersection(self, below_event: Event, event: Event, /) -> bool:
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
                        min_end_start_event, min_end_max_end_event = self._divide(
                            max_end_event, min_end
                        )
                        self._push(min_end_start_event)
                        self._push(min_end_max_end_event)
                    self._have_overlap[left_event_to_position(below_event)] = True
                    self._have_overlap[left_event_to_position(event)] = True
                    return True
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
            elif below_event_start_orientation is not below_event_end_orientation:
                cross_point = self._segments_intersector(
                    event_start, event_end, below_event_start, below_event_end
                )
                assert event_start < cross_point < event_end
                assert below_event_start < cross_point < below_event_end
                self._divide_event_by_midpoint(below_event, cross_point)
                self._divide_event_by_midpoint(event, cross_point)
        return False

    def _divide(
        self, event: Event, mid_point: hints.Point[hints.Scalar], /
    ) -> tuple[Event, Event]:
        assert is_event_left(event)
        opposite_event = self._to_opposite_event(event)
        mid_point_to_event_end_event: Event = Event(len(self.endpoints))
        self._segments_ids.append(self._left_event_to_segment_id(event))
        self.endpoints.append(mid_point)
        self._opposites.append(opposite_event)
        self._opposites[opposite_event] = mid_point_to_event_end_event
        self._other_have_interior_to_left.append(False)
        self._are_from_result.append(False)
        self._have_overlap.append(False)
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        assert self._is_left_event_from_first_operand(
            event
        ) is self._is_event_from_first_operand(mid_point_to_event_start_event)
        assert self._is_left_event_from_first_operand(
            event
        ) is self._is_left_event_from_first_operand(mid_point_to_event_end_event)
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

    def _events_to_connectivity(self, events: t.Sequence[Event], /) -> list[int]:
        events_count = len(events)
        result = [0] * events_count
        event_id = 0
        while event_id < events_count:
            current_start = self.to_event_start(events[event_id])
            right_start_event_id = event_id
            while (
                event_id < events_count
                and self.to_event_start(events[event_id]) == current_start
                and not is_event_left(events[event_id])
            ):
                event_id += 1
            left_start_event_id = event_id
            while (
                event_id < events_count
                and self.to_event_start(events[event_id]) == current_start
            ):
                event_id += 1
            left_stop_event_id = event_id - 1
            has_right_events = left_start_event_id >= right_start_event_id + 1
            has_left_events = left_stop_event_id >= left_start_event_id
            if has_right_events:
                result[right_start_event_id : left_start_event_id - 1] = range(
                    right_start_event_id + 1, left_start_event_id - 1 + 1
                )
                result[left_start_event_id - 1] = (
                    left_stop_event_id if has_left_events else right_start_event_id
                )
            if has_left_events:
                result[left_start_event_id] = (
                    right_start_event_id if has_right_events else left_stop_event_id
                )
                result[left_start_event_id + 1 : left_stop_event_id + 1] = range(
                    left_start_event_id, left_stop_event_id
                )
        return result

    def _find(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        candidate = self._sweep_line_data.tree.find(self._to_sweep_line_key(event))
        return None if candidate is red_black.NIL else candidate.value

    def _is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(self._to_left_event(event))

    def _is_event_from_result(self, event: Event, /) -> bool:
        return self._are_from_result[left_event_to_position(self._to_left_event(event))]

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
        return self._left_event_to_segment_id(event) < self.first_segments_count

    def _is_left_event_outside(self, event: Event, /) -> bool:
        event_position = left_event_to_position(event)
        return (
            not self._other_have_interior_to_left[event_position]
            and not self._have_overlap[event_position]
        )

    def _left_event_to_segment_id(self, event: Event, /) -> int:
        return self._segments_ids[left_event_to_position(event)]

    def _pop(self) -> Event:
        return self._events_queue_data.pop()

    def _process_contour_events(
        self,
        contour_events: t.Sequence[Event],
        contour_id: int,
        are_events_processed: list[bool],
        are_from_in_to_out: list[bool],
        contours_ids: list[int],
        events_ids: t.Sequence[int],
        /,
    ) -> None:
        for event in contour_events:
            are_events_processed[events_ids[event]] = True
            are_events_processed[events_ids[self._to_opposite_event(event)]] = True
            if is_event_left(event):
                are_from_in_to_out[events_ids[event]] = False
                contours_ids[events_ids[event]] = contour_id
            else:
                are_from_in_to_out[events_ids[self._to_opposite_event(event)]] = True
                contours_ids[events_ids[self._to_opposite_event(event)]] = contour_id

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.remove(event)

    def _to_left_event(self, event: Event, /) -> Event:
        return event if is_event_left(event) else self._to_opposite_event(event)

    def _to_opposite_event(self, event: Event, /) -> Event:
        return self._opposites[event]

    def _to_sweep_line_key(self, event: Event, /) -> SweepLineKey[hints.Scalar]:
        return SweepLineKey(
            event,
            self._is_left_event_from_first_operand(event),
            self.endpoints,
            self._opposites,
            self._orienteer,
        )


def _populate_with_segments(
    segments: t.Iterable[hints.Segment[hints.Scalar]],
    endpoints: list[hints.Point[hints.Scalar]],
    have_interior_to_left: list[bool],
    /,
) -> None:
    for segment in segments:
        start, end = segment.start, segment.end
        if start > end:
            start, end = end, start
            have_interior_to_left.append(False)
        else:
            have_interior_to_left.append(True)
        endpoints.append(start)
        endpoints.append(end)
