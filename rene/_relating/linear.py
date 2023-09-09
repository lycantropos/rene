from __future__ import annotations

import typing as t
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
                         square,
                         to_segments_intersection_point,
                         to_sorted_pair)
from .event import (Event,
                    is_event_left,
                    is_event_right,
                    left_event_to_position)
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey


def dot_multiply(first_start: hints.Point[hints.Scalar],
                 first_end: hints.Point[hints.Scalar],
                 second_start: hints.Point[hints.Scalar],
                 second_end: hints.Point[hints.Scalar]) -> hints.Scalar:
    return ((first_end.x - first_start.x) * (second_end.x - second_start.x)
            + (first_end.y - first_start.y) * (second_end.y - second_start.y))


def has_two_elements(iterator: t.Iterator[t.Any]) -> bool:
    return (next(iterator, None) is not None
            and next(iterator, None) is not None)


def is_point_in_angle(point: hints.Point[hints.Scalar],
                      vertex: hints.Point[hints.Scalar],
                      first_ray_point: hints.Point[hints.Scalar],
                      second_ray_point: hints.Point[hints.Scalar],
                      angle_orientation: Orientation) -> bool:
    first_half_orientation = orient(vertex, first_ray_point, point)
    second_half_orientation = orient(vertex, point, second_ray_point)
    return (second_half_orientation == angle_orientation
            if first_half_orientation is Orientation.COLLINEAR
            else (first_half_orientation is angle_orientation
                  if second_half_orientation is Orientation.COLLINEAR
                  else ((first_half_orientation is second_half_orientation)
                        and first_half_orientation
                        is (Orientation.COUNTERCLOCKWISE
                            if angle_orientation is Orientation.COLLINEAR
                            else angle_orientation))))


def squared_distance(start: hints.Point[hints.Scalar],
                     end: hints.Point[hints.Scalar]) -> hints.Scalar:
    return square(start.x - end.x) + square(start.y - end.y)


class Operation(t.Generic[hints.Scalar]):
    @classmethod
    def from_segments_iterables(
            cls,
            first: t.Iterable[hints.Segment[hints.Scalar]],
            second: t.Iterable[hints.Segment[hints.Scalar]],
            /
    ) -> te.Self:
        endpoints: t.List[hints.Point[hints.Scalar]] = []
        _populate_with_segments(first, endpoints)
        first_segments_count = len(endpoints) >> 1
        _populate_with_segments(second, endpoints)
        second_segments_count = (len(endpoints) >> 1) - first_segments_count
        return cls(first_segments_count, second_segments_count, endpoints)

    def has_crossing(self, same_start_events: t.Sequence[Event]) -> bool:
        if len(same_start_events) < 4:
            return False
        from_first_operand_events_count = sum(
                self.is_event_from_first_operand(event)
                for event in same_start_events
        )
        if not (1 < from_first_operand_events_count
                < len(same_start_events) - 1):
            # for crossing angles there should be
            # at least two pairs of segments from each operand
            return False
        from_first_events: t.List[Event] = []
        from_second_events: t.List[Event] = []
        for event in same_start_events:
            (from_first_events
             if self.is_event_from_first_operand(event)
             else from_second_events).append(event)
        start = self.to_event_start(same_start_events[0])
        base_event = min(
                from_second_events,
                key=lambda event:
                self.to_signed_point_event_squared_cosine(
                        self.to_event_end(from_second_events[0]), event,
                )
        )
        base_end = self.to_event_end(base_event)
        largest_angle_event = min(
                from_second_events,
                key=lambda event: self.to_signed_point_event_squared_cosine(
                        base_end, event
                )
        )
        largest_angle_end = self.to_event_end(largest_angle_event)
        base_orientation = orient(start, base_end, largest_angle_end)
        return not all_equal(
                is_point_in_angle(self.to_event_end(event), start, base_end,
                                  largest_angle_end, base_orientation)
                for event in from_first_events
        )

    def has_intersection(self, same_start_events: t.Sequence[Event]) -> bool:
        return not all_equal(self.is_event_from_first_operand(event)
                             for event in same_start_events)

    def is_event_from_first_operand(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
                self._to_left_event(event)
        )

    def to_event_end(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event, /) -> hints.Point[hints.Scalar]:
        return self.endpoints[event]

    def to_relation(self,
                    first_is_subset: bool,
                    second_is_subset: bool,
                    min_max_x: hints.Scalar,
                    /) -> Relation:
        state: RelationState[hints.Scalar] = RelationState(
                first_is_subset=first_is_subset,
                second_is_subset=second_is_subset,
                has_crossing=False,
                has_intersection=False,
                has_overlap=False
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
                if (state.has_overlap
                        and not state.first_is_subset
                        and not state.second_is_subset):
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
        if state.first_is_subset:
            if state.second_is_subset:
                return Relation.EQUAL
            else:
                return Relation.COMPONENT
        elif state.second_is_subset:
            return Relation.COMPOSITE
        elif state.has_overlap:
            return Relation.OVERLAP
        elif state.has_crossing:
            return Relation.CROSS
        elif state.has_intersection:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT

    _sweep_line_data: KeyedSet[SweepLineKey[hints.Scalar], Event]

    __slots__ = (
        'first_segments_count', 'second_segments_count', 'endpoints',
        '_events_queue_data', '_opposites', '_segments_ids', '_sweep_line_data'
    )

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __init__(self,
                 first_segments_count: int,
                 second_segments_count: int,
                 endpoints: t.List[hints.Point[hints.Scalar]],
                 /) -> None:
        (
            self.endpoints, self.first_segments_count,
            self.second_segments_count
        ) = endpoints, first_segments_count, second_segments_count
        segments_count = first_segments_count + second_segments_count
        initial_events_count = 2 * segments_count
        self._opposites = [Event(((index >> 1) << 1) + is_even(index))
                           for index in range(initial_events_count)]
        self._segments_ids = list(range(segments_count))
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[hints.Scalar], Event
        ] = PriorityQueue(
                *map(Event, range(initial_events_count)),
                key=lambda event: EventsQueueKey(
                        event, self.is_event_from_first_operand(event),
                        self.endpoints, self._opposites
                )
        )
        self._sweep_line_data = red_black.set_(key=self._to_sweep_line_key)

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

    def _detect_intersection(
            self, below_event: Event, event: Event, /
    ) -> None:
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
                    self._divide_event_by_midpoint(event, below_event_start)
            elif below_event_end_orientation is Orientation.COLLINEAR:
                if event_start < below_event_end < event_end:
                    self._divide_event_by_midpoint(event, below_event_end)
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
        mid_point_to_event_start_event = Event(len(self.endpoints))
        self.endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        assert (self._is_left_event_from_first_operand(event)
                is self.is_event_from_first_operand(
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

    def _is_left_event_from_first_operand(self, event: Event, /) -> bool:
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
            above_event, below_event = self._above(event), self._below(event)
            if above_event is not None:
                self._detect_intersection(event, above_event)
            if below_event is not None:
                self._detect_intersection(below_event, event)

    def _push(self, event: Event, /) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event, /) -> None:
        assert is_event_left(event)
        self._sweep_line_data.remove(event)

    def _to_event_endpoints(
            self, event: Event, /
    ) -> t.Tuple[hints.Point[hints.Scalar], hints.Point[hints.Scalar]]:
        return self.to_event_start(event), self.to_event_end(event)

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

    def to_signed_point_event_squared_cosine(self,
                                             point: hints.Point[hints.Scalar],
                                             event: Event) -> hints.Scalar:
        start = self.to_event_start(event)
        end = self.to_event_end(event)
        dot_product = dot_multiply(start, point, start, end)
        return ((square(dot_product)
                 if dot_product > 0
                 else -square(dot_product))
                / squared_distance(start, end))


class RelationState(t.Generic[hints.Scalar]):
    def update(self,
               same_start_events: t.List[Event],
               operation: Operation[hints.Scalar]) -> None:
        if operation.has_intersection(same_start_events):
            if not self.has_intersection:
                self.has_intersection = True
            self._detect_touch_or_overlap(same_start_events, operation)
            self._detect_crossing(same_start_events, operation)
        elif operation.is_event_from_first_operand(same_start_events[0]):
            assert all(operation.is_event_from_first_operand(event)
                       for event in same_start_events)
            if self.first_is_subset:
                self.first_is_subset = False
        elif self.second_is_subset:
            assert all(not operation.is_event_from_first_operand(event)
                       for event in same_start_events)
            self.second_is_subset = False

    def _detect_crossing(self,
                         same_start_events: t.Sequence[Event],
                         operation: Operation[hints.Scalar]) -> None:
        if not self.has_crossing and operation.has_crossing(same_start_events):
            self.has_crossing = True

    def _detect_touch_or_overlap(self,
                                 same_start_events: t.Sequence[Event],
                                 operation: Operation[hints.Scalar]) -> None:
        for _, group in chain(groupby(filter(is_event_left, same_start_events),
                                      key=operation.to_event_end)):
            event = next(group)
            if next(group, None) is not None:
                assert next(group, None) is None
                if not self.has_overlap:
                    self.has_overlap = True
            elif operation.is_event_from_first_operand(event):
                if self.first_is_subset:
                    self.first_is_subset = False
            elif self.second_is_subset:
                self.second_is_subset = False

    first_is_subset: bool
    has_crossing: bool
    has_intersection: bool
    has_overlap: bool
    second_is_subset: bool

    __slots__ = ('first_is_subset', 'second_is_subset', 'has_crossing',
                 'has_intersection', 'has_overlap')

    def __init__(self,
                 *,
                 first_is_subset: bool,
                 second_is_subset: bool,
                 has_crossing: bool,
                 has_intersection: bool,
                 has_overlap: bool) -> None:
        (
            self.first_is_subset, self.second_is_subset, self.has_crossing,
            self.has_intersection, self.has_overlap
        ) = (
            first_is_subset, second_is_subset, has_crossing, has_intersection,
            has_overlap
        )


def _populate_with_segments(
        segments: t.Iterable[hints.Segment[hints.Scalar]],
        endpoints: t.List[hints.Point[hints.Scalar]],
        /
) -> None:
    for segment in segments:
        start, end = to_sorted_pair(segment.start, segment.end)
        endpoints.append(start)
        endpoints.append(end)
