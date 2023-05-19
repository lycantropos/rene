from __future__ import annotations

import typing as _t
from abc import (ABC,
                 abstractmethod)

from dendroid import red_black
from dendroid.hints import KeyedSet
from prioq.base import PriorityQueue
from reprit.base import generate_repr

from rene import (Orientation as _Orientation,
                  hints as _hints)
from rene._utils import (intersect_crossing_segments,
                         is_even,
                         is_odd,
                         orient,
                         shrink_collinear_vertices)
from .constants import UNDEFINED_INDEX
from .event import (UNDEFINED_EVENT,
                    Event,
                    is_left_event,
                    is_right_event,
                    left_event_to_position)
from .events_queue_key import BinaryEventsQueueKey as EventsQueueKey
from .overlap_kind import OverlapKind
from .sweep_line_key import BinarySweepLineKey as SweepLineKey


class Operation(ABC, _t.Generic[_hints.Scalar]):
    @classmethod
    def from_segments_iterables(
            cls,
            first: _t.Iterable[_hints.Segment[_hints.Scalar]],
            second: _t.Iterable[_hints.Segment[_hints.Scalar]]
    ) -> Operation[_hints.Scalar]:
        endpoints: _t.List[_hints.Point[_hints.Scalar]] = []
        have_interior_to_left: _t.List[bool] = []
        _populate_with_segments(first, endpoints, have_interior_to_left)
        first_segments_count = len(have_interior_to_left)
        _populate_with_segments(second, endpoints, have_interior_to_left)
        second_segments_count = (len(have_interior_to_left)
                                 - first_segments_count)
        self = cls(first_segments_count, second_segments_count, endpoints,
                   have_interior_to_left)
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    @property
    def segments_count(self) -> int:
        return self._first_segments_count + self._second_segments_count

    def reduce_events(
            self,
            events: _t.List[Event],
            contour_cls: _t.Type[_hints.Contour[_hints.Scalar]],
            polygon_cls: _t.Type[_hints.Polygon[_hints.Scalar]]
    ) -> _t.List[_hints.Polygon[_hints.Scalar]]:
        events = [event
                  for event in events
                  if self._is_from_result_event(event)]
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
        are_internal: _t.List[bool] = []
        connectivity = self._events_to_connectivity(events)
        contours_ids = [UNDEFINED_INDEX] * len(events)
        contours_vertices: _t.List[_t.List[_hints.Point[_hints.Scalar]]] = []
        depths: _t.List[int] = []
        holes: _t.List[_t.List[int]] = []
        parents: _t.List[int] = []
        visited_endpoints_positions = ([UNDEFINED_INDEX]
                                       * self._unique_visited_endpoints_count)
        for event_id, event in enumerate(events):
            if are_events_processed[event_id]:
                continue
            contour_id = len(contours_vertices)
            self._compute_relations(event, contour_id, are_internal, depths,
                                    holes, parents, are_from_in_to_out,
                                    contours_ids, events_ids)
            contour_events = self._to_contour_events(
                    event, events, events_ids, connectivity,
                    are_events_processed, visited_endpoints_positions
            )
            self._process_contour_events(
                    contour_events, contour_id, are_events_processed,
                    are_from_in_to_out, contours_ids, events_ids
            )
            vertices = self._contour_events_to_vertices(contour_events)
            if is_odd(depths[contour_id]):
                vertices = vertices[:1] + vertices[:0:-1]
            contours_vertices.append(vertices)
        result: _t.List[_hints.Polygon[_hints.Scalar]] = []
        for contour_id, contour_vertices in enumerate(contours_vertices):
            if are_internal[contour_id]:
                # hole of a hole is an external polygon
                result.extend(
                        polygon_cls(
                                contour_cls(contours_vertices[hole_id]),
                                [
                                    contour_cls(
                                            contours_vertices[hole_hole_id]
                                    )
                                    for hole_hole_id in holes[hole_id]
                                ]
                        )
                        for hole_id in holes[contour_id]
                )
            else:
                result.append(
                        polygon_cls(contour_cls(contours_vertices[contour_id]),
                                    [contour_cls(contours_vertices[hole_id])
                                     for hole_id in holes[contour_id]])
                )
        return result

    def to_event_end(self, event: Event) -> _hints.Point[_hints.Scalar]:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event) -> _hints.Point[_hints.Scalar]:
        return self._endpoints[event]

    _sweep_line_data: KeyedSet[SweepLineKey[_hints.Scalar], Event]

    __slots__ = (
        '_first_segments_count', '_are_from_result',
        '_other_have_interior_to_left', '_below_event_from_result',
        '_current_endpoint_first_event', '_current_endpoint_id', '_endpoints',
        '_events_queue_data', '_have_interior_to_left', '_opposites',
        '_overlap_kinds', '_segments_ids', '_starts_ids', '_sweep_line_data'
    )

    def __init__(self,
                 _first_segments_count: int,
                 _second_segments_count: int,
                 _endpoints: _t.List[_hints.Point[_hints.Scalar]],
                 _have_interior_to_left: _t.Sequence[bool]) -> None:
        segments_count = _first_segments_count + _second_segments_count
        initial_events_count = 2 * segments_count
        self._first_segments_count = _first_segments_count
        self._second_segments_count = _second_segments_count
        self._are_from_result = [False] * segments_count
        self._below_event_from_result = [UNDEFINED_EVENT] * segments_count
        self._current_endpoint_first_event = UNDEFINED_EVENT
        self._current_endpoint_id = 0
        self._endpoints = _endpoints
        self._have_interior_to_left = _have_interior_to_left
        self._opposites = [Event(((index >> 1) << 1) + is_even(index))
                           for index in range(initial_events_count)]
        self._other_have_interior_to_left = [False] * segments_count
        self._overlap_kinds = [OverlapKind.NONE] * segments_count
        self._segments_ids = list(range(segments_count))
        self._starts_ids: _t.List[int] = (
                [UNDEFINED_INDEX] * initial_events_count
        )
        self._events_queue_data: PriorityQueue[
            EventsQueueKey[_hints.Scalar], Event
        ] = PriorityQueue(
                *range(initial_events_count),
                key=lambda event: EventsQueueKey(
                        event,
                        self._is_from_first_operand_event(event),
                        self._endpoints, self._opposites
                )
        )
        self._sweep_line_data = red_black.set_(key=self._to_sweep_line_key)

    __repr__ = generate_repr(__init__)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self) -> _t.Iterator[Event]:
        while self:
            event = self._pop()
            if (self.to_event_start(self._current_endpoint_first_event)
                    != self.to_event_start(event)):
                self._current_endpoint_first_event = event
                self._current_endpoint_id += 1
            self._starts_ids[event] = self._current_endpoint_id
            if is_right_event(event):
                opposite_event = self._to_opposite_event(event)
                assert is_left_event(opposite_event)
                equal_segment_event = self._find(opposite_event)
                if equal_segment_event is not None:
                    above_event, below_event = (
                        self._above(equal_segment_event),
                        self._below(equal_segment_event)
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
                yield event

    @property
    def _events_count(self) -> int:
        return len(self._endpoints)

    @property
    def _unique_visited_endpoints_count(self) -> int:
        return self._current_endpoint_id + 1

    @abstractmethod
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        pass

    def _above(self, event: Event) -> _t.Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.next(event)
        except ValueError:
            return None

    def _add(self, event: Event) -> None:
        assert is_left_event(event)
        self._sweep_line_data.add(event)

    def _below(self, event: Event) -> _t.Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.prev(event)
        except ValueError:
            return None

    def _compute_left_event_fields(self,
                                   event: Event,
                                   below_event: _t.Optional[Event]) -> None:
        event_position = left_event_to_position(event)
        if below_event is not None:
            below_event_position = left_event_to_position(below_event)
            self._other_have_interior_to_left[event_position] = (
                self._other_have_interior_to_left[below_event_position]
                if (self._is_left_event_from_first_operand(event)
                    is self._is_left_event_from_first_operand(below_event))
                else self._have_interior_to_left[
                    self._left_event_to_segment_id(below_event)
                ]
            )
            self._below_event_from_result[event_position] = (
                self._below_event_from_result[below_event_position]
                if (not self._detect_if_left_event_from_result(below_event)
                    or self._is_vertical_left_event(below_event))
                else below_event
            )
        self._are_from_result[
            event_position
        ] = self._detect_if_left_event_from_result(event)

    def _compute_relations(
            self,
            event: Event,
            contour_id: int,
            are_internal: _t.List[bool],
            depths: _t.List[int],
            holes: _t.List[_t.List[int]],
            parents: _t.List[int],
            are_from_in_to_out: _t.Sequence[bool],
            contours_ids: _t.Sequence[int],
            events_ids: _t.Sequence[int],
    ) -> None:
        assert is_left_event(event)
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
            self, events: _t.Sequence[Event]
    ) -> _t.List[_hints.Point[_hints.Scalar]]:
        result = ([self.to_event_start(events[0])]
                  + [self.to_event_end(event) for event in events[:-1]])
        return shrink_collinear_vertices(result)

    def _detect_intersection(self, below_event: Event, event: Event) -> bool:
        event_start = self.to_event_start(event)
        event_end = self.to_event_end(event)
        below_event_start = self.to_event_start(below_event)
        below_event_end = self.to_event_end(below_event)
        event_start_orientation = orient(below_event_end, below_event_start,
                                         event_start)
        event_end_orientation = orient(below_event_end, below_event_start,
                                       event_end)
        if (event_start_orientation is not _Orientation.COLLINEAR
                and event_end_orientation is not _Orientation.COLLINEAR):
            if event_start_orientation is not event_end_orientation:
                below_event_start_orientation = orient(event_start, event_end,
                                                       below_event_start)
                below_event_end_orientation = orient(event_start, event_end,
                                                     below_event_end)
                if (below_event_start_orientation is not _Orientation.COLLINEAR
                        and (below_event_end_orientation
                             is not _Orientation.COLLINEAR)):
                    if (below_event_start_orientation
                            is not below_event_end_orientation):
                        point = intersect_crossing_segments(
                                event_start, event_end, below_event_start,
                                below_event_end
                        )
                        assert event_start < point < event_end
                        assert below_event_start < point < below_event_end
                        self._divide_event_by_midpoint(below_event, point)
                        self._divide_event_by_midpoint(event, point)
                elif below_event_start_orientation != _Orientation.COLLINEAR:
                    if event_start < below_event_end < event_end:
                        point = below_event_end
                        self._divide_event_by_midpoint(event, point)
                elif event_start < below_event_start < event_end:
                    point = below_event_start
                    self._divide_event_by_midpoint(event, point)
        elif event_end_orientation is not _Orientation.COLLINEAR:
            if below_event_start < event_start < below_event_end:
                point = event_start
                self._divide_event_by_midpoint(below_event, point)
        elif event_start_orientation is not _Orientation.COLLINEAR:
            if below_event_start < event_end < below_event_end:
                point = event_end
                self._divide_event_by_midpoint(below_event, point)
        else:
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
                    min_end_start_event, min_end_max_end_event = self._divide(
                            max_end_event, min_end
                    )
                    self._push(min_end_start_event)
                    self._push(min_end_max_end_event)
                overlap_kind = (
                    OverlapKind.SAME_ORIENTATION
                    if self._have_interior_to_left[
                           self._left_event_to_segment_id(event)
                       ] is self._have_interior_to_left[
                           self._left_event_to_segment_id(below_event)
                       ]
                    else OverlapKind.DIFFERENT_ORIENTATION
                )
                self._overlap_kinds[
                    left_event_to_position(below_event)
                ] = overlap_kind
                self._overlap_kinds[
                    left_event_to_position(event)
                ] = overlap_kind
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
                                                    max_start,
                                                    min_end)
            elif event_start < below_event_start < event_end:
                if below_event_end < event_end:
                    self._divide_event_by_mid_segment_event_endpoints(
                            event, below_event_start, below_event_end
                    )
                else:
                    max_start, min_end = below_event_start, event_end
                    self._divide_overlapping_events(event, below_event,
                                                    max_start,
                                                    min_end)
        return False

    def _divide(
            self, event: Event, mid_point: _hints.Point[_hints.Scalar]
    ) -> _t.Tuple[Event, Event]:
        assert is_left_event(event)
        opposite_event = self._to_opposite_event(event)
        mid_point_to_event_end_event: Event = Event(len(self._endpoints))
        self._segments_ids.append(self._left_event_to_segment_id(event))
        self._endpoints.append(mid_point)
        self._opposites.append(opposite_event)
        self._opposites[opposite_event] = mid_point_to_event_end_event
        self._other_have_interior_to_left.append(False)
        self._are_from_result.append(False)
        self._below_event_from_result.append(UNDEFINED_EVENT)
        self._overlap_kinds.append(OverlapKind.NONE)
        self._starts_ids.append(UNDEFINED_INDEX)
        mid_point_to_event_start_event = Event(len(self._endpoints))
        self._endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        self._starts_ids.append(UNDEFINED_INDEX)
        assert (self._is_left_event_from_first_operand(event)
                is self._is_from_first_operand_event(
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
            mid_segment_event_start: _hints.Point[_hints.Scalar],
            mid_segment_event_end: _hints.Point[_hints.Scalar]
    ) -> None:
        self._divide_event_by_midpoint(event, mid_segment_event_end)
        self._divide_event_by_midpoint(event, mid_segment_event_start)

    def _divide_event_by_midpoint(self, event: Event,
                                  point: _hints.Point[_hints.Scalar]) -> None:
        point_to_event_start_event, point_to_event_end_event = self._divide(
                event, point
        )
        self._push(point_to_event_start_event)
        self._push(point_to_event_end_event)

    def _divide_overlapping_events(
            self,
            min_start_event: Event,
            max_start_event: Event,
            max_start: _hints.Point[_hints.Scalar],
            min_end: _hints.Point[_hints.Scalar]
    ) -> None:
        self._divide_event_by_midpoint(max_start_event, min_end)
        self._divide_event_by_midpoint(min_start_event, max_start)

    def _events_to_connectivity(self,
                                events: _t.Sequence[Event]) -> _t.List[int]:
        events_count = len(events)
        result = [0] * events_count
        event_id = 0
        while event_id < events_count:
            current_start = self.to_event_start(events[event_id])
            right_start_event_id = event_id
            while (event_id < events_count
                   and self.to_event_start(events[event_id]) == current_start
                   and not is_left_event(events[event_id])):
                event_id += 1
            left_start_event_id = event_id
            while (event_id < events_count
                   and self.to_event_start(events[event_id]) == current_start):
                event_id += 1
            left_stop_event_id = event_id - 1
            has_right_events = left_start_event_id >= right_start_event_id + 1
            has_left_events = left_stop_event_id >= left_start_event_id
            if has_right_events:
                result[right_start_event_id:left_start_event_id - 1] = range(
                        right_start_event_id + 1, left_start_event_id - 1 + 1
                )
                result[left_start_event_id - 1] = (left_stop_event_id
                                                   if has_left_events
                                                   else right_start_event_id)
            if has_left_events:
                result[left_start_event_id] = (right_start_event_id
                                               if has_right_events
                                               else left_stop_event_id)
                result[left_start_event_id + 1:left_stop_event_id + 1] = range(
                        left_start_event_id, left_stop_event_id
                )
        return result

    def _find(self, event: Event) -> _t.Optional[Event]:
        assert is_left_event(event)
        candidate = self._sweep_line_data.tree.find(
                self._to_sweep_line_key(event)
        )
        return (None
                if candidate is red_black.NIL
                else candidate.value)

    def _is_common_polyline_component_left_event(self, event: Event) -> bool:
        return (self._overlap_kinds[left_event_to_position(event)]
                is OverlapKind.DIFFERENT_ORIENTATION)

    def _is_common_region_boundary_left_event(self, event: Event) -> bool:
        return (self._overlap_kinds[left_event_to_position(event)]
                is OverlapKind.SAME_ORIENTATION)

    def _is_from_first_operand_event(self, event: Event) -> bool:
        return self._is_left_event_from_first_operand(
                self._to_left_event(event)
        )

    def _is_from_result_event(self, event: Event) -> bool:
        return self._are_from_result[
            left_event_to_position(self._to_left_event(event))
        ]

    def _is_inside_left_event(self, event: Event) -> bool:
        event_position = left_event_to_position(event)
        return (self._other_have_interior_to_left[event_position]
                and self._overlap_kinds[event_position] is OverlapKind.NONE)

    def _is_left_event_from_first_operand(self, event: Event) -> bool:
        return (self._left_event_to_segment_id(event)
                < self._first_segments_count)

    def _is_outside_left_event(self, event: Event) -> bool:
        event_position = left_event_to_position(event)
        return (not self._other_have_interior_to_left[event_position]
                and self._overlap_kinds[event_position] is OverlapKind.NONE)

    def _is_overlap_left_event(self, event: Event) -> bool:
        return (self._overlap_kinds[left_event_to_position(event)]
                is not OverlapKind.NONE)

    def _is_vertical_left_event(self, event: Event) -> bool:
        assert is_left_event(event)
        return self.to_event_start(event).x == self.to_event_end(event).x

    def _left_event_to_segment_id(self, event: Event) -> int:
        return self._segments_ids[left_event_to_position(event)]

    def _peek(self) -> Event:
        return self._events_queue_data.peek()

    def _pop(self) -> Event:
        return self._events_queue_data.pop()

    def _process_contour_events(self,
                                contour_events: _t.Sequence[Event],
                                contour_id: int,
                                are_events_processed: _t.List[bool],
                                are_from_in_to_out: _t.List[bool],
                                contours_ids: _t.List[int],
                                events_ids: _t.Sequence[int]) -> None:
        for event in contour_events:
            are_events_processed[events_ids[event]] = True
            are_events_processed[
                events_ids[self._to_opposite_event(event)]
            ] = True
            if is_left_event(event):
                are_from_in_to_out[events_ids[event]] = False
                contours_ids[events_ids[event]] = contour_id
            else:
                are_from_in_to_out[
                    events_ids[self._to_opposite_event(event)]
                ] = True
                contours_ids[
                    events_ids[self._to_opposite_event(event)]
                ] = contour_id

    def _push(self, event: Event) -> None:
        self._events_queue_data.push(event)

    def _remove(self, event: Event) -> None:
        assert is_left_event(event)
        self._sweep_line_data.remove(event)

    def _to_contour_events(
            self,
            event: Event,
            events: _t.Sequence[Event],
            events_ids: _t.Sequence[int],
            connectivity: _t.Sequence[int],
            are_events_processed: _t.Sequence[bool],
            visited_endpoints_positions: _t.List[int]
    ) -> _t.List[Event]:
        assert is_left_event(event)
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
            event_id = _to_next_event_id(opposite_event_id,
                                         are_events_processed, connectivity)
            if event_id == UNDEFINED_INDEX:
                break
            cursor = events[event_id]
            opposite_event_id = events_ids[self._to_opposite_event(cursor)]
            result.append(cursor)
        visited_endpoints_positions[self._to_start_id(event)] = UNDEFINED_INDEX
        for endpoint_id in visited_endpoints_ids:
            visited_endpoints_positions[endpoint_id] = UNDEFINED_INDEX
        assert all(position == UNDEFINED_INDEX
                   for position in visited_endpoints_positions)
        return result

    def _to_end_id(self, event: Event) -> int:
        return self._starts_ids[self._to_opposite_event(event)]

    def _to_left_event(self, event: Event) -> Event:
        return (event
                if is_left_event(event)
                else self._to_opposite_event(event))

    def _to_opposite_event(self, event: Event) -> Event:
        return self._opposites[event]

    def _to_start_id(self, event: Event) -> int:
        return self._starts_ids[event]

    def _to_sweep_line_key(self, event: Event) -> SweepLineKey[_hints.Scalar]:
        return SweepLineKey(
                event, self._is_left_event_from_first_operand(event),
                self._endpoints, self._opposites
        )


def _populate_with_segments(
        segments: _t.Iterable[_hints.Segment[_hints.Scalar]],
        endpoints: _t.List[_hints.Point[_hints.Scalar]],
        have_interior_to_left: _t.List[bool]
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


def _multisegmentals_to_segments_count(
        multisegmentals: _t.Sequence[
            _hints.Multisegmental[_hints.Segment[_hints.Scalar]]
        ]
) -> int:
    return sum(multisegment.segments_count for multisegment in multisegmentals)


def _to_next_event_id(event_id: int,
                      are_events_processed: _t.Sequence[bool],
                      connectivity: _t.Sequence[int]) -> int:
    candidate = event_id
    while True:
        candidate = connectivity[candidate]
        if not are_events_processed[candidate]:
            return candidate
        elif candidate == event_id:
            return UNDEFINED_INDEX
