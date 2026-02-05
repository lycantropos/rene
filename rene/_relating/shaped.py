from __future__ import annotations

import enum
from typing import Generic, TYPE_CHECKING

from dendroid import red_black
from prioq.base import PriorityQueue
from typing_extensions import Self

from rene import hints
from rene._utils import all_same, is_even
from rene.enums import Orientation, Relation

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
    from collections.abc import Iterable, Sequence

    from dendroid.hints import Map

    from rene._hints import Orienteer, SegmentsIntersector


class Operation(Generic[hints.ScalarT]):
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
        return cls(
            first_segments_count,
            second_segments_count,
            endpoints,
            have_interior_to_left,
            orienteer,
            segments_intersector,
        )

    def classify_event(self, event: Event, /) -> EventKind:
        left_event_position = left_event_to_position(
            self._to_left_event(event)
        )
        overlap_kind = self._overlap_kinds[left_event_position]
        if overlap_kind is OverlapKind.NONE:
            return (
                EventKind.INSIDE
                if self._other_have_interior_to_left[left_event_position]
                else EventKind.OUTSIDE
            )
        if overlap_kind is OverlapKind.DIFFERENT_ORIENTATION:
            return EventKind.COMMON_POLYLINE_SEGMENT
        assert overlap_kind is OverlapKind.SAME_ORIENTATION, overlap_kind
        return EventKind.COMMON_REGION_EDGE

    def has_edges_cross(self, same_start_events: Iterable[Event], /) -> bool:
        flags: list[bool | None] = [None, None]
        for event in same_start_events:
            left_event = self._to_left_event(event)
            left_event_position = left_event_to_position(left_event)
            if self._overlap_kinds[left_event_position] is OverlapKind.NONE:
                flag_index = self._is_left_event_from_first_operand(left_event)
                previous_event_is_inside = flags[flag_index]
                event_is_inside = self._other_have_interior_to_left[
                    left_event_position
                ]
                if previous_event_is_inside is None:
                    flags[flag_index] = event_is_inside
                elif previous_event_is_inside is not event_is_inside:
                    return True
        return False

    def is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
            self._to_left_event(event)
        )

    def to_event_end(self, event: Event, /) -> hints.Point[hints.ScalarT]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.ScalarT]:
        return self.endpoints[event]

    def to_relation(
        self,
        /,
        *,
        first_is_subset: bool,
        second_is_subset: bool,
        min_max_x: hints.ScalarT,
    ) -> Relation:
        state: RelationState[hints.ScalarT] = RelationState(
            boundaries_intersect=False,
            first_boundary_intersects_second_interior=False,
            first_is_subset=first_is_subset,
            second_boundary_intersects_first_interior=False,
            second_is_subset=second_is_subset,
            has_continuous_intersection=False,
        )
        event = self._pop()
        previous_start = self.to_event_start(event)
        same_start_events = [event]
        self._process_event(event)
        while self:
            event = self._pop()
            start = self.to_event_start(event)
            if start == previous_start:
                same_start_events.append(event)
            else:
                state.update(same_start_events, self)
                same_start_events.clear()
                if (
                    state.has_continuous_intersection
                    and not state.first_is_subset
                    and not state.second_is_subset
                ):
                    break
                if start.x > min_max_x:
                    if self.is_event_from_first_operand(event):
                        if state.first_is_subset:
                            state.first_is_subset = False
                    elif state.second_is_subset:
                        state.second_is_subset = False
                    break
                previous_start = start
                same_start_events.append(event)
            self._process_event(event)
        else:
            assert same_start_events
            state.update(same_start_events, self)
            same_start_events.clear()
        assert not same_start_events, same_start_events
        if state.boundaries_intersect:
            if state.first_is_subset:
                return (
                    Relation.EQUAL
                    if state.second_is_subset
                    else (
                        Relation.ENCLOSED
                        if state.first_boundary_intersects_second_interior
                        else Relation.COMPONENT
                    )
                )
            if state.second_is_subset:
                return (
                    Relation.ENCLOSES
                    if state.second_boundary_intersects_first_interior
                    else Relation.COMPOSITE
                )
            return (
                Relation.OVERLAP
                if state.has_continuous_intersection
                else Relation.TOUCH
            )
        return (
            Relation.COVER
            if state.second_is_subset
            else (
                Relation.WITHIN
                if state.first_is_subset
                else (
                    Relation.OVERLAP
                    if state.has_continuous_intersection
                    else Relation.DISJOINT
                )
            )
        )

    _sweep_line_data: Map[SweepLineKey[hints.ScalarT], Event]

    __slots__ = (
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
        self._opposites = [
            Event(((index >> 1) << 1) + is_even(index))
            for index in range(initial_events_count)
        ]
        self._other_have_interior_to_left = [False] * segments_count
        self._overlap_kinds = [OverlapKind.NONE] * segments_count
        self._segments_ids = list(range(segments_count))
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[hints.ScalarT], Event
        ] = PriorityQueue(
            *map(Event, range(initial_events_count)),
            key=lambda event: EventsQueueKey(
                event,
                is_from_first_operand=self.is_event_from_first_operand(event),
                endpoints=self.endpoints,
                opposites=self._opposites,
                orienteer=self._orienteer,
            ),
        )
        self._sweep_line_data = red_black.map_()

    def __bool__(self, /) -> bool:
        return bool(self._events_queue_data)

    def _above(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.next(self._to_sweep_line_key(event))
        except ValueError:
            return None

    def _add(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data[self._to_sweep_line_key(event)] = event

    def _below(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.prev(self._to_sweep_line_key(event))
        except ValueError:
            return None

    def _compute_left_event_fields(
        self, event: Event, below_event: Event | None, /
    ) -> None:
        if below_event is not None:
            self._other_have_interior_to_left[
                left_event_to_position(event)
            ] = (
                self._other_have_interior_to_left[
                    left_event_to_position(below_event)
                ]
                if (
                    self._is_left_event_from_first_operand(event)
                    is self._is_left_event_from_first_operand(below_event)
                )
                else self.have_interior_to_left[
                    self._left_event_to_segment_id(below_event)
                ]
            )

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
                    self._overlap_kinds[
                        left_event_to_position(below_event)
                    ] = self._overlap_kinds[left_event_to_position(event)] = (
                        OverlapKind.SAME_ORIENTATION
                        if self.have_interior_to_left[
                            self._left_event_to_segment_id(event)
                        ]
                        is self.have_interior_to_left[
                            self._left_event_to_segment_id(below_event)
                        ]
                        else OverlapKind.DIFFERENT_ORIENTATION
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
        self._overlap_kinds.append(OverlapKind.NONE)
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        assert self._is_left_event_from_first_operand(
            event
        ) is self.is_event_from_first_operand(mid_point_to_event_start_event)
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

    def _find(self, event: Event, /) -> Event | None:
        assert is_event_left(event)
        return self._sweep_line_data.get(self._to_sweep_line_key(event))

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
        return (
            self._left_event_to_segment_id(event) < self.first_segments_count
        )

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

    def _process_event(self, event: Event) -> None:
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
                self._compute_left_event_fields(below_event, below_below_event)
                self._compute_left_event_fields(event, below_event)

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        del self._sweep_line_data[self._to_sweep_line_key(event)]

    def _to_left_event(self, event: Event, /) -> Event:
        return (
            event if is_event_left(event) else self._to_opposite_event(event)
        )

    def _to_opposite_event(self, event: Event, /) -> Event:
        return self._opposites[event]

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


class EventKind(enum.IntEnum):
    COMMON_POLYLINE_SEGMENT = enum.auto()
    COMMON_REGION_EDGE = enum.auto()
    INSIDE = enum.auto()
    OUTSIDE = enum.auto()


class OverlapKind(enum.IntEnum):
    NONE = 0
    SAME_ORIENTATION = 1
    DIFFERENT_ORIENTATION = 2


class RelationState(Generic[hints.ScalarT]):
    def update(
        self,
        same_start_events: list[Event],
        operation: Operation[hints.ScalarT],
    ) -> None:
        if all_same(
            operation.is_event_from_first_operand(event)
            for event in same_start_events
        ):
            event = same_start_events[0]
            event_kind = operation.classify_event(event)
            if event_kind is EventKind.INSIDE:
                if not self.has_continuous_intersection:
                    self.has_continuous_intersection = True
                if operation.is_event_from_first_operand(event):
                    if not self.first_boundary_intersects_second_interior:
                        self.first_boundary_intersects_second_interior = True
                    if self.second_is_subset:
                        self.second_is_subset = False
                else:
                    if not self.second_boundary_intersects_first_interior:
                        self.second_boundary_intersects_first_interior = True
                    if self.first_is_subset:
                        self.first_is_subset = False
            else:
                assert event_kind is EventKind.OUTSIDE, event_kind
                if operation.is_event_from_first_operand(event):
                    if self.first_is_subset:
                        self.first_is_subset = False
                elif self.second_is_subset:
                    self.second_is_subset = False
        elif operation.has_edges_cross(same_start_events):
            self.has_continuous_intersection = True
            self.first_is_subset = False
            self.second_is_subset = False
        else:
            if not self.boundaries_intersect:
                self.boundaries_intersect = True
            for event in same_start_events:
                event_kind = operation.classify_event(event)
                if event_kind is EventKind.COMMON_REGION_EDGE:
                    if not self.has_continuous_intersection:
                        self.has_continuous_intersection = True
                elif event_kind is EventKind.INSIDE:
                    if not self.has_continuous_intersection:
                        self.has_continuous_intersection = True
                    if operation.is_event_from_first_operand(event):
                        if not self.first_boundary_intersects_second_interior:
                            self.first_boundary_intersects_second_interior = (
                                True
                            )
                        if self.second_is_subset:
                            self.second_is_subset = False
                    else:
                        if not self.second_boundary_intersects_first_interior:
                            self.second_boundary_intersects_first_interior = (
                                True
                            )
                        if self.first_is_subset:
                            self.first_is_subset = False
                else:
                    assert (
                        event_kind is EventKind.OUTSIDE
                        or event_kind is EventKind.COMMON_POLYLINE_SEGMENT
                    ), event_kind
                    if operation.is_event_from_first_operand(event):
                        if self.first_is_subset:
                            self.first_is_subset = False
                    elif self.second_is_subset:
                        self.second_is_subset = False

    boundaries_intersect: bool
    first_boundary_intersects_second_interior: bool
    first_is_subset: bool
    has_continuous_intersection: bool
    second_boundary_intersects_first_interior: bool
    second_is_subset: bool

    __slots__ = (
        'boundaries_intersect',
        'first_boundary_intersects_second_interior',
        'first_is_subset',
        'has_continuous_intersection',
        'second_boundary_intersects_first_interior',
        'second_is_subset',
    )

    def __init__(
        self,
        *,
        boundaries_intersect: bool,
        first_boundary_intersects_second_interior: bool,
        first_is_subset: bool,
        has_continuous_intersection: bool,
        second_boundary_intersects_first_interior: bool,
        second_is_subset: bool,
    ) -> None:
        (
            self.boundaries_intersect,
            self.first_boundary_intersects_second_interior,
            self.first_is_subset,
            self.has_continuous_intersection,
            self.second_boundary_intersects_first_interior,
            self.second_is_subset,
        ) = (
            boundaries_intersect,
            first_boundary_intersects_second_interior,
            first_is_subset,
            has_continuous_intersection,
            second_boundary_intersects_first_interior,
            second_is_subset,
        )


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
