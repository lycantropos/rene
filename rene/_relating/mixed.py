from __future__ import annotations

import typing as t
from abc import (ABC,
                 abstractmethod)
from itertools import (chain,
                       groupby)

import typing_extensions as te
from dendroid import red_black
from dendroid.hints import KeyedSet
from prioq.base import PriorityQueue

from rene import (Orientation,
                  Relation,
                  hints)
from rene._utils import (all_equal,
                         is_even,
                         orient,
                         to_segments_intersection_point)
from .event import (Event,
                    is_event_left,
                    is_event_right,
                    left_event_to_position)
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey

SegmentEndpoints = t.Tuple[
    hints.Point[hints.Scalar], hints.Point[hints.Scalar]]


class Operation(ABC, t.Generic[hints.Scalar]):
    @classmethod
    @abstractmethod
    def from_segments_iterables(
            cls,
            first: t.Iterable[hints.Segment[hints.Scalar]],
            second: t.Iterable[hints.Segment[hints.Scalar]],
            reverse_shaped_orientation: bool,
            /
    ) -> te.Self:
        ...

    def has_border_intersection(self, same_start_events: t.List[Event],
                                /) -> bool:
        return not all_equal(self._is_event_from_first_operand(event)
                             for event in same_start_events)

    @abstractmethod
    def is_event_from_linear(self, event: Event, /) -> bool:
        ...

    def is_left_event_inside(self, event: Event, /) -> bool:
        return self._other_have_interior_to_left[left_event_to_position(event)]

    def is_left_event_outside(self, event: Event, /) -> bool:
        return not self._other_have_interior_to_left[
            left_event_to_position(event)
        ]

    def is_event_inside(self, event: Event, /) -> bool:
        return self.is_left_event_inside(self._to_left_event(event))

    def is_event_outside(self, event: Event, /) -> bool:
        return self.is_left_event_outside(self._to_left_event(event))

    def to_event_end(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.endpoints[event]

    def to_relation(self,
                    linear_is_subset_of_shaped: bool,
                    min_max_x: hints.Scalar,
                    /) -> Relation:
        state: RelationState[hints.Scalar] = RelationState(
                linear_is_subset_of_shaped=linear_is_subset_of_shaped,
                shaped_border_is_subset_of_linear=True,
                linear_intersects_shaped_interior=False,
                linear_intersects_shaped_border=False,
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
                if (state.linear_intersects_shaped_interior
                        and not state.linear_is_subset_of_shaped
                        and not state.shaped_border_is_subset_of_linear):
                    break
                if start.x > min_max_x:
                    if self.is_event_from_linear(event):
                        if (state.linear_is_subset_of_shaped
                                and self.is_event_outside(event)):
                            state.linear_is_subset_of_shaped = False
                        if (not state.linear_intersects_shaped_interior
                                and self.is_event_inside(event)):
                            state.linear_intersects_shaped_interior = True
                    elif state.shaped_border_is_subset_of_linear:
                        state.shaped_border_is_subset_of_linear = False
                    break
                previous_start = start
                same_start_events.append(event)
            self._process_event(event)
        else:
            assert same_start_events
            state.update(same_start_events, self)
            same_start_events.clear()
        assert not same_start_events, same_start_events
        if state.shaped_border_is_subset_of_linear:
            return ((Relation.ENCLOSED
                     if state.linear_intersects_shaped_interior
                     else Relation.COMPONENT)
                    if state.linear_is_subset_of_shaped
                    else (Relation.CROSS
                          if state.linear_intersects_shaped_interior
                          else Relation.TOUCH))
        elif state.linear_is_subset_of_shaped:
            return ((Relation.ENCLOSED
                     if state.linear_intersects_shaped_border
                     else Relation.WITHIN)
                    if state.linear_intersects_shaped_interior
                    else Relation.COMPONENT)
        else:
            return (Relation.CROSS
                    if state.linear_intersects_shaped_interior
                    else (Relation.TOUCH
                          if state.linear_intersects_shaped_border
                          else Relation.DISJOINT))

    _sweep_line_data: KeyedSet[SweepLineKey[hints.Scalar], Event]

    __slots__ = (
        'endpoints', 'first_segments_count', 'have_interior_to_left',
        'second_segments_count', '_events_queue_data', '_opposites',
        '_other_have_interior_to_left', '_segments_ids', '_sweep_line_data'
    )

    def __init__(self,
                 first_segments_count: int,
                 second_segments_count: int,
                 endpoints: t.List[hints.Point[hints.Scalar]],
                 have_interior_to_left: t.Sequence[bool],
                 /) -> None:
        (
            self.endpoints, self.first_segments_count,
            self.have_interior_to_left, self.second_segments_count
        ) = (endpoints, first_segments_count, have_interior_to_left,
             second_segments_count)
        segments_count = first_segments_count + second_segments_count
        initial_events_count = 2 * segments_count
        self._opposites = [Event(((index >> 1) << 1) + is_even(index))
                           for index in range(initial_events_count)]
        self._other_have_interior_to_left = [False] * segments_count
        self._segments_ids = list(range(segments_count))
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[hints.Scalar], Event
        ] = PriorityQueue(
                *map(Event, range(initial_events_count)),
                key=lambda event: EventsQueueKey(
                        event, self._is_event_from_first_operand(event),
                        self.endpoints, self._opposites
                )
        )
        self._sweep_line_data = red_black.set_(key=self._to_sweep_line_key)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def _above(self, event: Event, /) -> t.Optional[Event]:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.next(event)
        except ValueError:
            return None

    def _add(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.add(event)

    def _below(self, event: Event, /) -> t.Optional[Event]:
        assert is_event_left(event)
        try:
            return self._sweep_line_data.prev(event)
        except ValueError:
            return None

    def _compute_left_event_fields(
            self, event: Event, below_event: t.Optional[Event], /
    ) -> None:
        if below_event is not None:
            self._other_have_interior_to_left[
                left_event_to_position(event)
            ] = (
                self._other_have_interior_to_left[
                    left_event_to_position(below_event)
                ]
                if (self._is_left_event_from_first_operand(event)
                    is self._is_left_event_from_first_operand(below_event))
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
        event_start_orientation = orient(below_event_end, below_event_start,
                                         event_start)
        event_end_orientation = orient(below_event_end, below_event_start,
                                       event_end)
        if event_start_orientation is event_end_orientation:
            if event_start_orientation is Orientation.COLLINEAR:
                assert (self._is_left_event_from_first_operand(below_event)
                        is not self._is_left_event_from_first_operand(event))
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
                    return True
                elif event_end == below_event_end:
                    max_start_event, min_start_event = (
                        (below_event, event)
                        if event_start < below_event_start
                        else (event, below_event)
                    )
                    max_start = self.to_event_start(max_start_event)
                    (
                        max_start_to_min_start_event, max_start_to_end_event
                    ) = self._divide(min_start_event, max_start)
                    self._push(max_start_to_min_start_event)
                    self._push(max_start_to_end_event)
                elif below_event_start < event_start < below_event_end:
                    if event_end < below_event_end:
                        self._divide_event_by_mid_segment_event_endpoints(
                                below_event, event_start, event_end
                        )
                    else:
                        max_start, min_end = event_start, below_event_end
                        self._divide_overlapping_events(below_event, event,
                                                        max_start, min_end)
                elif event_start < below_event_start < event_end:
                    if below_event_end < event_end:
                        self._divide_event_by_mid_segment_event_endpoints(
                                event, below_event_start, below_event_end
                        )
                    else:
                        max_start, min_end = below_event_start, event_end
                        self._divide_overlapping_events(event, below_event,
                                                        max_start, min_end)
        elif event_start_orientation is Orientation.COLLINEAR:
            if below_event_start < event_start < below_event_end:
                point = event_start
                self._divide_event_by_midpoint(below_event, point)
        elif event_end_orientation is Orientation.COLLINEAR:
            if below_event_start < event_end < below_event_end:
                point = event_end
                self._divide_event_by_midpoint(below_event, point)
        else:
            below_event_start_orientation = orient(event_start, event_end,
                                                   below_event_start)
            below_event_end_orientation = orient(event_start, event_end,
                                                 below_event_end)
            if below_event_start_orientation is Orientation.COLLINEAR:
                assert below_event_end_orientation is not Orientation.COLLINEAR
                if event_start < below_event_start < event_end:
                    point = below_event_start
                    self._divide_event_by_midpoint(event, point)
            elif below_event_end_orientation is Orientation.COLLINEAR:
                if event_start < below_event_end < event_end:
                    point = below_event_end
                    self._divide_event_by_midpoint(event, point)
            elif (below_event_start_orientation
                  is not below_event_end_orientation):
                cross_point = to_segments_intersection_point(
                        event_start, event_end, below_event_start,
                        below_event_end
                )
                assert event_start < cross_point < event_end
                assert below_event_start < cross_point < below_event_end
                self._divide_event_by_midpoint(below_event, cross_point)
                self._divide_event_by_midpoint(event, cross_point)
        return False

    def _divide(
            self, event: Event, mid_point: hints.Point[hints.Scalar], /
    ) -> t.Tuple[Event, Event]:
        assert is_event_left(event)
        opposite_event = self._to_opposite_event(event)
        mid_point_to_event_end_event: Event = Event(len(self.endpoints))
        self._segments_ids.append(self._left_event_to_segment_id(event))
        self.endpoints.append(mid_point)
        self._opposites.append(opposite_event)
        self._opposites[opposite_event] = mid_point_to_event_end_event
        self._other_have_interior_to_left.append(False)
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        assert (self._is_left_event_from_first_operand(event)
                is self._is_event_from_first_operand(
                        mid_point_to_event_start_event
                ))
        assert (self._is_left_event_from_first_operand(event)
                is self._is_left_event_from_first_operand(
                        mid_point_to_event_end_event
                ))
        return mid_point_to_event_start_event, mid_point_to_event_end_event

    def _divide_event_by_mid_segment_event_endpoints(
            self,
            event: Event,
            mid_segment_event_start: hints.Point[hints.Scalar],
            mid_segment_event_end: hints.Point[hints.Scalar],
            /
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
            /
    ) -> None:
        self._divide_event_by_midpoint(max_start_event, min_end)
        self._divide_event_by_midpoint(min_start_event, max_start)

    def _find(self, event: Event, /) -> t.Optional[Event]:
        assert is_event_left(event)
        candidate = self._sweep_line_data.tree.find(
                self._to_sweep_line_key(event)
        )
        return (None
                if candidate is red_black.NIL
                else candidate.value)

    def _is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
                self._to_left_event(event)
        )

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
        assert is_event_left(event), event
        return (self._left_event_to_segment_id(event)
                < self.first_segments_count)

    def _left_event_to_segment_id(self, event: Event, /) -> int:
        return self._segments_ids[left_event_to_position(event)]

    def _pop(self) -> Event:
        return self._events_queue_data.pop()

    def _process_event(self, event: Event) -> None:
        if is_event_right(event):
            opposite_event = self._to_opposite_event(event)
            assert is_event_left(opposite_event)
            equal_segment_event = self._find(opposite_event)
            if equal_segment_event is not None:
                above_event, below_event = (
                    self._above(equal_segment_event),
                    self._below(equal_segment_event)
                )
                self._remove(equal_segment_event)
                if below_event is not None and above_event is not None:
                    self._detect_intersection(below_event, above_event)
        elif self._find(event) is None:
            self._add(event)
            above_event, below_event = (self._above(event),
                                        self._below(event))
            self._compute_left_event_fields(event, below_event)
            if above_event is not None:
                if self._detect_intersection(event, above_event):
                    self._compute_left_event_fields(event, below_event)
                    self._compute_left_event_fields(above_event, event)
            if below_event is not None:
                if self._detect_intersection(below_event, event):
                    below_below_event = self._below(below_event)
                    self._compute_left_event_fields(below_event,
                                                    below_below_event)
                    self._compute_left_event_fields(event, below_event)

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.remove(event)

    def _to_left_event(self, event: Event, /) -> Event:
        return (event
                if is_event_left(event)
                else self._to_opposite_event(event))

    def _to_opposite_event(self, event: Event, /) -> Event:
        return self._opposites[event]

    def _to_sweep_line_key(
            self, event: Event, /
    ) -> SweepLineKey[hints.Scalar]:
        return SweepLineKey(
                event, self._is_left_event_from_first_operand(event),
                self.endpoints, self._opposites
        )


class LinearShapedOperation(Operation[hints.Scalar]):
    @classmethod
    def from_segments_iterables(
            cls,
            first: t.Iterable[hints.Segment[hints.Scalar]],
            second: t.Iterable[hints.Segment[hints.Scalar]],
            reverse_shaped_orientation: bool,
            /
    ) -> te.Self:
        endpoints: t.List[hints.Point[hints.Scalar]] = []
        have_interior_to_left: t.List[bool] = []
        _populate_with_linear_segments(first, endpoints, have_interior_to_left)
        first_segments_count = len(have_interior_to_left)
        _populate_with_shaped_segments(second, endpoints,
                                       have_interior_to_left,
                                       reverse_shaped_orientation)
        second_segments_count = (len(have_interior_to_left)
                                 - first_segments_count)
        return cls(first_segments_count, second_segments_count, endpoints,
                   have_interior_to_left)

    def is_event_from_linear(self, event: Event, /) -> bool:
        return self._is_event_from_first_operand(event)


class ShapedLinearOperation(Operation[hints.Scalar]):
    @classmethod
    def from_segments_iterables(
            cls,
            first: t.Iterable[hints.Segment[hints.Scalar]],
            second: t.Iterable[hints.Segment[hints.Scalar]],
            reverse_shaped_orientation: bool,
            /
    ) -> te.Self:
        endpoints: t.List[hints.Point[hints.Scalar]] = []
        have_interior_to_left: t.List[bool] = []
        _populate_with_shaped_segments(first, endpoints, have_interior_to_left,
                                       reverse_shaped_orientation)
        first_segments_count = len(have_interior_to_left)
        _populate_with_linear_segments(second, endpoints,
                                       have_interior_to_left)
        second_segments_count = (len(have_interior_to_left)
                                 - first_segments_count)
        return cls(first_segments_count, second_segments_count, endpoints,
                   have_interior_to_left)

    def is_event_from_linear(self, event: Event, /) -> bool:
        return not self._is_left_event_from_first_operand(event)


class RelationState(t.Generic[hints.Scalar]):
    def update(self,
               same_start_events: t.List[Event],
               operation: Operation[hints.Scalar]) -> None:
        if operation.has_border_intersection(same_start_events):
            if not self.linear_intersects_shaped_border:
                self.linear_intersects_shaped_border = True
            for _, group in chain(groupby(filter(is_event_left,
                                                 same_start_events),
                                          key=operation.to_event_end)):
                event = next(group)
                if next(group, None) is not None:
                    assert next(group, None) is None
                elif operation.is_event_from_linear(event):
                    if (self.linear_is_subset_of_shaped
                            and operation.is_left_event_outside(event)):
                        self.linear_is_subset_of_shaped = False
                    if (not self.linear_intersects_shaped_interior
                            and operation.is_left_event_inside(event)):
                        self.linear_intersects_shaped_interior = True
                elif self.shaped_border_is_subset_of_linear:
                    self.shaped_border_is_subset_of_linear = False
        elif operation.is_event_from_linear(same_start_events[0]):
            assert all(operation.is_event_from_linear(event)
                       for event in same_start_events)
            if (self.linear_is_subset_of_shaped
                    and operation.is_event_outside(same_start_events[0])):
                self.linear_is_subset_of_shaped = False
            if (not self.linear_intersects_shaped_interior
                    and operation.is_event_inside(same_start_events[0])):
                self.linear_intersects_shaped_interior = True
        elif self.shaped_border_is_subset_of_linear:
            assert all(not operation.is_event_from_linear(event)
                       for event in same_start_events)
            self.shaped_border_is_subset_of_linear = False

    linear_intersects_shaped_border: bool
    linear_intersects_shaped_interior: bool
    linear_is_subset_of_shaped: bool
    shaped_border_is_subset_of_linear: bool

    __slots__ = (
        'linear_intersects_shaped_interior', 'linear_is_subset_of_shaped',
        'linear_intersects_shaped_border', 'shaped_border_is_subset_of_linear'
    )

    def __init__(self,
                 *,
                 linear_intersects_shaped_border: bool,
                 linear_intersects_shaped_interior: bool,
                 linear_is_subset_of_shaped: bool,
                 shaped_border_is_subset_of_linear: bool) -> None:
        (
            self.linear_intersects_shaped_border,
            self.linear_intersects_shaped_interior,
            self.linear_is_subset_of_shaped,
            self.shaped_border_is_subset_of_linear
        ) = (
            linear_intersects_shaped_border, linear_intersects_shaped_interior,
            linear_is_subset_of_shaped, shaped_border_is_subset_of_linear
        )


def _populate_with_shaped_segments(
        segments: t.Iterable[hints.Segment[hints.Scalar]],
        endpoints: t.List[hints.Point[hints.Scalar]],
        have_interior_to_left: t.List[bool],
        reverse_orientation: bool,
        /
) -> None:
    for segment in segments:
        start, end = segment.start, segment.end
        if start > end:
            start, end = end, start
            have_interior_to_left.append(reverse_orientation)
        else:
            have_interior_to_left.append(not reverse_orientation)
        endpoints.append(start)
        endpoints.append(end)


def _populate_with_linear_segments(
        segments: t.Iterable[hints.Segment[hints.Scalar]],
        endpoints: t.List[hints.Point[hints.Scalar]],
        have_interior_to_left: t.List[bool],
        /
) -> None:
    offset = len(endpoints)
    for segment in segments:
        start, end = segment.start, segment.end
        if start > end:
            start, end = end, start
        endpoints.append(start)
        endpoints.append(end)
    segments_count = (len(endpoints) - offset) // 2
    have_interior_to_left.extend([False] * segments_count)
