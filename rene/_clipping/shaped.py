from __future__ import annotations

import enum
from abc import ABC, abstractmethod
from typing import Generic, TYPE_CHECKING

from dendroid import red_black
from prioq.base import PriorityQueue
from typing_extensions import Self

from rene import hints
from rene._utils import is_even, is_odd, shrink_collinear_vertices
from rene.enums import Orientation

from .constants import UNDEFINED_INDEX
from .event import (
    Event,
    UNDEFINED_EVENT,
    is_event_left,
    is_event_right,
    left_event_to_position,
)
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey

if TYPE_CHECKING:
    from collections.abc import Iterable, Iterator, Sequence

    from dendroid.hints import Map

    from rene._hints import Orienteer, SegmentsIntersector


class Operation(ABC, Generic[hints.ScalarT]):
    @classmethod
    def from_segments_iterables(
        cls,
        first: Iterable[hints.Segment[hints.ScalarT]],
        second: Iterable[hints.Segment[hints.ScalarT]],
        orienteer: Orienteer[hints.ScalarT],
        segments_intersector: SegmentsIntersector[hints.ScalarT],
        /,
    ) -> Self:
        endpoints: list[hints.Point[hints.ScalarT]] = []
        have_interior_to_left: list[bool] = []
        _populate_with_segments(first, endpoints, have_interior_to_left)
        first_segments_count = len(have_interior_to_left)
        _populate_with_segments(second, endpoints, have_interior_to_left)
        second_segments_count = (
            len(have_interior_to_left) - first_segments_count
        )
        self = cls(
            first_segments_count,
            second_segments_count,
            endpoints,
            have_interior_to_left,
            orienteer,
            segments_intersector,
        )
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    def reduce_events(
        self,
        events: list[Event],
        contour_cls: type[hints.Contour[hints.ScalarT]],
        polygon_cls: type[hints.Polygon[hints.ScalarT]],
        /,
    ) -> list[hints.Polygon[hints.ScalarT]]:
        events = [
            event for event in events if self._is_event_from_result(event)
        ]
        if not events:
            return []
        events.sort(key=self._events_queue_data.key)
        events_ids = [UNDEFINED_INDEX] * self._events_count
        for event_id, event in enumerate(events):
            events_ids[event] = event_id
        assert all(
            events_ids[self._to_opposite_event(event)] != UNDEFINED_INDEX
            for event in events
        )
        are_events_processed = [False] * len(events)
        are_from_in_to_out = [False] * len(events)
        are_internal: list[bool] = []
        connectivity = self._events_to_connectivity(events)
        contours_ids = [UNDEFINED_INDEX] * len(events)
        contours_vertices: list[list[hints.Point[hints.ScalarT]]] = []
        depths: list[int] = []
        holes: list[list[int]] = []
        parents: list[int] = []
        visited_endpoints_positions = [
            UNDEFINED_INDEX
        ] * self._unique_visited_endpoints_count
        for event_id, event in enumerate(events):
            if are_events_processed[event_id]:
                continue
            contour_id = len(contours_vertices)
            self._compute_relations(
                event,
                contour_id,
                are_internal,
                depths,
                holes,
                parents,
                are_from_in_to_out,
                contours_ids,
                events_ids,
            )
            contour_events = self._to_contour_events(
                event,
                events,
                events_ids,
                connectivity,
                are_events_processed,
                visited_endpoints_positions,
            )
            self._process_contour_events(
                contour_events,
                contour_id,
                are_events_processed,
                are_from_in_to_out,
                contours_ids,
                events_ids,
            )
            vertices = self._contour_events_to_vertices(contour_events)
            if is_odd(depths[contour_id]):
                vertices = vertices[:1] + vertices[:0:-1]
            contours_vertices.append(vertices)
        result: list[hints.Polygon[hints.ScalarT]] = []
        for contour_id in range(len(contours_vertices)):
            if are_internal[contour_id]:
                # hole of a hole is an external polygon
                result.extend(
                    polygon_cls(
                        contour_cls(contours_vertices[hole_id]),
                        [
                            contour_cls(contours_vertices[hole_hole_id])
                            for hole_hole_id in holes[hole_id]
                        ],
                    )
                    for hole_id in holes[contour_id]
                )
            else:
                result.append(
                    polygon_cls(
                        contour_cls(contours_vertices[contour_id]),
                        [
                            contour_cls(contours_vertices[hole_id])
                            for hole_id in holes[contour_id]
                        ],
                    )
                )
        return result

    def to_event_end(self, event: Event, /) -> hints.Point[hints.ScalarT]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.ScalarT]:
        return self.endpoints[event]

    _sweep_line_data: Map[SweepLineKey[hints.ScalarT], Event]

    __slots__ = (
        '_are_from_result',
        '_below_event_from_result',
        '_current_endpoint_first_event',
        '_current_endpoint_id',
        '_events_queue_data',
        '_opposites',
        '_orienteer',
        '_other_have_interior_to_left',
        '_overlap_kinds',
        '_segments_ids',
        '_segments_intersector',
        '_starts_ids',
        '_sweep_line_data',
        'endpoints',
        'first_segments_count',
        'have_interior_to_left',
        'second_segments_count',
    )

    def __init__(
        self,
        first_segments_count: int,
        second_segments_count: int,
        endpoints: list[hints.Point[hints.ScalarT]],
        have_interior_to_left: Sequence[bool],
        orienteer: Orienteer[hints.ScalarT],
        segments_intersector: SegmentsIntersector[hints.ScalarT],
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
        self._below_event_from_result = [UNDEFINED_EVENT] * segments_count
        self._current_endpoint_first_event = UNDEFINED_EVENT
        self._current_endpoint_id = 0
        self._opposites = [
            Event(((index >> 1) << 1) + is_even(index))
            for index in range(initial_events_count)
        ]
        self._other_have_interior_to_left = [False] * segments_count
        self._overlap_kinds = [OverlapKind.NONE] * segments_count
        self._segments_ids = list(range(segments_count))
        self._starts_ids: list[int] = [UNDEFINED_INDEX] * initial_events_count
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[hints.ScalarT], Event
        ] = PriorityQueue(
            *map(Event, range(initial_events_count)),
            key=lambda event: EventsQueueKey(
                event,
                is_from_first_operand=self._is_event_from_first_operand(event),
                endpoints=self.endpoints,
                opposites=self._opposites,
                orienteer=self._orienteer,
            ),
        )
        self._sweep_line_data = red_black.map_()

    def __bool__(self, /) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self, /) -> Iterator[Event]:
        while self:
            event = self._pop()
            if self.to_event_start(
                self._current_endpoint_first_event
            ) != self.to_event_start(event):
                self._current_endpoint_first_event = event
                self._current_endpoint_id += 1
            self._starts_ids[event] = self._current_endpoint_id
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
                below_event = self._below(event)
                self._compute_left_event_fields(event, below_event)
                above_event = self._above(event)
                if above_event is not None and self._detect_intersection(
                    event, above_event
                ):
                    self._compute_left_event_fields(event, below_event)
                    self._compute_left_event_fields(above_event, event)
                if below_event is not None and self._detect_intersection(
                    below_event, event
                ):
                    below_below_event = self._below(below_event)
                    self._compute_left_event_fields(
                        below_event, below_below_event
                    )
                    self._compute_left_event_fields(event, below_event)
                yield event

    @property
    def _events_count(self, /) -> int:
        return len(self.endpoints)

    @property
    def _unique_visited_endpoints_count(self, /) -> int:
        return self._current_endpoint_id + 1

    @abstractmethod
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        pass

    def _above(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.next(self._to_sweep_line_key(event))
        except KeyError:
            return None

    def _add(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data[self._to_sweep_line_key(event)] = event

    def _below(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.prev(self._to_sweep_line_key(event))
        except KeyError:
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
            self._below_event_from_result[event_position] = (
                self._below_event_from_result[below_event_position]
                if (
                    not self._detect_if_left_event_from_result(below_event)
                    or self._is_left_event_vertical(below_event)
                )
                else below_event
            )
        self._are_from_result[event_position] = (
            self._detect_if_left_event_from_result(event)
        )

    def _compute_relations(
        self,
        event: Event,
        contour_id: int,
        are_internal: list[bool],
        depths: list[int],
        holes: list[list[int]],
        parents: list[int],
        are_from_in_to_out: Sequence[bool],
        contours_ids: Sequence[int],
        events_ids: Sequence[int],
        /,
    ) -> None:
        assert is_event_left(event)
        depth = 0
        parent = UNDEFINED_INDEX
        is_internal = False
        below_event_from_result = self._below_event_from_result[
            left_event_to_position(event)
        ]
        if below_event_from_result != UNDEFINED_EVENT:
            below_event_from_result_id = events_ids[below_event_from_result]
            below_contour_id = contours_ids[below_event_from_result_id]
            if not are_from_in_to_out[below_event_from_result_id]:
                if not are_internal[below_contour_id]:
                    holes[below_contour_id].append(contour_id)
                    parent = below_contour_id
                    depth = depths[below_contour_id] + 1
                    is_internal = True
            elif are_internal[below_contour_id]:
                below_contour_parent = parents[below_contour_id]
                holes[below_contour_parent].append(contour_id)
                parent = below_contour_parent
                depth = depths[below_contour_id]
                is_internal = True
        holes.append([])
        parents.append(parent)
        depths.append(depth)
        are_internal.append(is_internal)

    def _contour_events_to_vertices(
        self, events: Sequence[Event], /
    ) -> list[hints.Point[hints.ScalarT]]:
        result = [self.to_event_start(events[0])] + [
            self.to_event_end(event) for event in events[:-1]
        ]
        return shrink_collinear_vertices(result, self._orienteer)

    def _detect_intersection(
        self, below_event: Event, event: Event, /
    ) -> bool:
        event_start = self.to_event_start(event)
        event_end = self.to_event_end(event)
        below_event_start = self.to_event_start(below_event)
        below_event_end = self.to_event_end(below_event)
        event_start_orientation = self._orienteer(
            below_event_start, below_event_end, event_start
        )
        event_end_orientation = self._orienteer(
            below_event_start, below_event_end, event_end
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
                    overlap_kind = (
                        OverlapKind.SAME_ORIENTATION
                        if self.have_interior_to_left[
                            self._left_event_to_segment_id(event)
                        ]
                        is self.have_interior_to_left[
                            self._left_event_to_segment_id(below_event)
                        ]
                        else OverlapKind.DIFFERENT_ORIENTATION
                    )
                    self._overlap_kinds[
                        left_event_to_position(below_event)
                    ] = overlap_kind
                    self._overlap_kinds[left_event_to_position(event)] = (
                        overlap_kind
                    )
                    return True
                if event_end == below_event_end:
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
        return False

    def _divide(
        self, event: Event, mid_point: hints.Point[hints.ScalarT], /
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
        self._below_event_from_result.append(UNDEFINED_EVENT)
        self._overlap_kinds.append(OverlapKind.NONE)
        self._starts_ids.append(UNDEFINED_INDEX)
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        self._starts_ids.append(UNDEFINED_INDEX)
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
        mid_segment_event_start: hints.Point[hints.ScalarT],
        mid_segment_event_end: hints.Point[hints.ScalarT],
        /,
    ) -> None:
        self._divide_event_by_midpoint(event, mid_segment_event_end)
        self._divide_event_by_midpoint(event, mid_segment_event_start)

    def _divide_event_by_midpoint(
        self, event: Event, point: hints.Point[hints.ScalarT], /
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
        max_start: hints.Point[hints.ScalarT],
        min_end: hints.Point[hints.ScalarT],
        /,
    ) -> None:
        self._divide_event_by_midpoint(max_start_event, min_end)
        self._divide_event_by_midpoint(min_start_event, max_start)

    def _events_to_connectivity(self, events: Sequence[Event], /) -> list[int]:
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
                    left_stop_event_id
                    if has_left_events
                    else right_start_event_id
                )
            if has_left_events:
                result[left_start_event_id] = (
                    right_start_event_id
                    if has_right_events
                    else left_stop_event_id
                )
                result[left_start_event_id + 1 : left_stop_event_id + 1] = (
                    range(left_start_event_id, left_stop_event_id)
                )
        return result

    def _find(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        return self._sweep_line_data.get(self._to_sweep_line_key(event))

    def _is_left_event_common_polyline_component(
        self, event: Event, /
    ) -> bool:
        return (
            self._overlap_kinds[left_event_to_position(event)]
            is OverlapKind.DIFFERENT_ORIENTATION
        )

    def _is_left_event_common_region_boundary(self, event: Event, /) -> bool:
        return (
            self._overlap_kinds[left_event_to_position(event)]
            is OverlapKind.SAME_ORIENTATION
        )

    def _is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
            self._to_left_event(event)
        )

    def _is_event_from_result(self, event: Event, /) -> bool:
        return self._are_from_result[
            left_event_to_position(self._to_left_event(event))
        ]

    def _is_left_event_inside(self, event: Event, /) -> bool:
        event_position = left_event_to_position(event)
        return (
            self._other_have_interior_to_left[event_position]
            and self._overlap_kinds[event_position] is OverlapKind.NONE
        )

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
        return (
            self._left_event_to_segment_id(event) < self.first_segments_count
        )

    def _is_left_event_outside(self, event: Event, /) -> bool:
        event_position = left_event_to_position(event)
        return (
            not self._other_have_interior_to_left[event_position]
            and self._overlap_kinds[event_position] is OverlapKind.NONE
        )

    def _is_left_event_overlapping(self, event: Event, /) -> bool:
        return (
            self._overlap_kinds[left_event_to_position(event)]
            is not OverlapKind.NONE
        )

    def _is_left_event_vertical(self, event: Event, /) -> bool:
        assert is_event_left(event)
        return self.to_event_start(event).x == self.to_event_end(event).x

    def _left_event_to_segment_id(self, event: Event, /) -> int:
        return self._segments_ids[left_event_to_position(event)]

    def _peek(self, /) -> Event:
        return (
            self._events_queue_data.peek()
            if self._events_queue_data
            else UNDEFINED_EVENT
        )

    def _pop(self, /) -> Event:
        return self._events_queue_data.pop()

    def _process_contour_events(
        self,
        contour_events: Sequence[Event],
        contour_id: int,
        are_events_processed: list[bool],
        are_from_in_to_out: list[bool],
        contours_ids: list[int],
        events_ids: Sequence[int],
        /,
    ) -> None:
        for event in contour_events:
            are_events_processed[events_ids[event]] = True
            are_events_processed[
                events_ids[self._to_opposite_event(event)]
            ] = True
            if is_event_left(event):
                are_from_in_to_out[events_ids[event]] = False
                contours_ids[events_ids[event]] = contour_id
            else:
                are_from_in_to_out[
                    events_ids[self._to_opposite_event(event)]
                ] = True
                contours_ids[events_ids[self._to_opposite_event(event)]] = (
                    contour_id
                )

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        del self._sweep_line_data[self._to_sweep_line_key(event)]

    def _to_contour_events(
        self,
        event: Event,
        events: Sequence[Event],
        events_ids: Sequence[int],
        connectivity: Sequence[int],
        are_events_processed: Sequence[bool],
        visited_endpoints_positions: list[int],
        /,
    ) -> list[Event]:
        assert is_event_left(event)
        result = [event]
        visited_endpoints_positions[self._to_start_id(event)] = 0
        opposite_event_id = events_ids[self._to_opposite_event(event)]
        cursor = event
        contour_start = self.to_event_start(event)
        visited_endpoints_ids = [self._to_start_id(event)]
        while self.to_event_end(cursor) != contour_start:
            previous_endpoint_position = visited_endpoints_positions[
                self._to_end_id(cursor)
            ]
            if previous_endpoint_position == UNDEFINED_INDEX:
                visited_endpoints_positions[self._to_end_id(cursor)] = len(
                    result
                )
            else:
                # vertices loop found, i.e. contour has self-intersection
                assert previous_endpoint_position != 0
                del result[previous_endpoint_position:]
            visited_endpoints_ids.append(self._to_end_id(cursor))
            event_id = _to_next_event_id(
                opposite_event_id, are_events_processed, connectivity
            )
            if event_id == UNDEFINED_INDEX:
                break
            cursor = events[event_id]
            opposite_event_id = events_ids[self._to_opposite_event(cursor)]
            result.append(cursor)
        visited_endpoints_positions[self._to_start_id(event)] = UNDEFINED_INDEX
        for endpoint_id in visited_endpoints_ids:
            visited_endpoints_positions[endpoint_id] = UNDEFINED_INDEX
        assert all(
            position == UNDEFINED_INDEX
            for position in visited_endpoints_positions
        )
        return result

    def _to_end_id(self, event: Event, /) -> int:
        return self._starts_ids[self._to_opposite_event(event)]

    def _to_left_event(self, event: Event, /) -> Event:
        return (
            event if is_event_left(event) else self._to_opposite_event(event)
        )

    def _to_opposite_event(self, event: Event, /) -> Event:
        return self._opposites[event]

    def _to_start_id(self, event: Event, /) -> int:
        return self._starts_ids[event]

    def _to_sweep_line_key(
        self, event: Event, /
    ) -> SweepLineKey[hints.ScalarT]:
        return SweepLineKey(
            event,
            is_from_first_operand=self._is_left_event_from_first_operand(
                event
            ),
            endpoints=self.endpoints,
            opposites=self._opposites,
            orienteer=self._orienteer,
        )


class OverlapKind(enum.IntEnum):
    NONE = 0
    SAME_ORIENTATION = 1
    DIFFERENT_ORIENTATION = 2


def _populate_with_segments(
    segments: Iterable[hints.Segment[hints.ScalarT]],
    endpoints: list[hints.Point[hints.ScalarT]],
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
        endpoints.extend((start, end))


def _to_next_event_id(
    event_id: int,
    are_events_processed: Sequence[bool],
    connectivity: Sequence[int],
    /,
) -> int:
    candidate = event_id
    while True:
        candidate = connectivity[candidate]
        if not are_events_processed[candidate]:
            return candidate
        if candidate == event_id:
            return UNDEFINED_INDEX
