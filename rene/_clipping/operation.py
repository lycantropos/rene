from abc import (ABC,
                 abstractmethod)
from typing import (Iterable,
                    Iterator,
                    List,
                    Optional,
                    Sequence,
                    Tuple,
                    Type)

from dendroid import red_black
from prioq.base import PriorityQueue
from reprit.base import generate_repr

from rene._rene import Orientation
from rene._utils import (intersect_crossing_segments,
                         orient,
                         shrink_collinear_vertices)
from rene.hints import (Contour,
                        Multisegmental,
                        Point,
                        Polygon,
                        Segment)
from .constants import UNDEFINED_INDEX
from .event import (UNDEFINED_EVENT,
                    Event,
                    is_left_event,
                    is_right_event,
                    left_event_to_position,
                    segment_id_to_left_event,
                    segment_id_to_right_event)
from .events_queue_key import EventsQueueKey
from .overlap_kind import OverlapKind
from .sweep_line_key import SweepLineKey


class Operation(ABC):
    @classmethod
    def from_multisegmentals(cls,
                             first: Multisegmental,
                             second: Multisegmental) -> 'Operation':
        first_segments_count = first.segments_count
        second_segments_count = second.segments_count
        segments_count = first_segments_count + second_segments_count
        self = cls(segments_count)
        self._are_from_first_operand.extend([True] * first_segments_count)
        self._are_from_first_operand.extend([False] * second_segments_count)
        self._extend(first.segments)
        self._extend(second.segments)
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    @classmethod
    def from_multisegmentals_sequences(
            cls,
            first: Sequence[Multisegmental],
            second: Sequence[Multisegmental]
    ) -> 'Operation':
        first_segments_count = _multisegmentals_to_segments_count(first)
        second_segments_count = _multisegmentals_to_segments_count(second)
        segments_count = first_segments_count + second_segments_count
        self = cls(segments_count)
        self._are_from_first_operand.extend([True] * first_segments_count)
        self._are_from_first_operand.extend([False] * second_segments_count)
        for multisegmental in first:
            self._extend(multisegmental.segments)
        for multisegmental in second:
            self._extend(multisegmental.segments)
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    @classmethod
    def from_multisegmental_multisegmentals_sequence(
            cls,
            first: Multisegmental,
            second: Sequence[Multisegmental]
    ) -> 'Operation':
        first_segments_count = first.segments_count
        second_segments_count = _multisegmentals_to_segments_count(second)
        segments_count = first_segments_count + second_segments_count
        self = cls(segments_count)
        self._are_from_first_operand.extend([True] * first_segments_count)
        self._are_from_first_operand.extend([False] * second_segments_count)
        self._extend(first.segments)
        for multisegmental in second:
            self._extend(multisegmental.segments)
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    @classmethod
    def from_multisegmentals_sequence_multisegmental(
            cls,
            first: Sequence[Multisegmental],
            second: Multisegmental
    ) -> 'Operation':
        first_segments_count = _multisegmentals_to_segments_count(first)
        second_segments_count = second.segments_count
        segments_count = first_segments_count + second_segments_count
        self = cls(segments_count)
        self._are_from_first_operand.extend([True] * first_segments_count)
        self._are_from_first_operand.extend([False] * second_segments_count)
        for multisegmental in first:
            self._extend(multisegmental.segments)
        self._extend(second.segments)
        first_event = self._peek()
        self._current_endpoint_first_event = first_event
        return self

    @property
    def segments_count(self) -> int:
        return len(self._have_interior_to_left)

    def reduce_events(self,
                      events: List[Event],
                      contour_cls: Type[Contour],
                      polygon_cls: Type[Polygon]) -> List[Polygon]:
        events = [event
                  for event in events
                  if self._is_from_result_event(event)]
        if not events:
            return []
        events.sort(key=self._events_queue_data.key)
        events_ids = [UNDEFINED_INDEX] * self._events_count
        for event_id, event in enumerate(events):
            events_ids[event] = event_id
        are_events_processed = [False] * len(events)
        are_from_in_to_out = [False] * len(events)
        are_internal: List[bool] = []
        connectivity = self._events_to_connectivity(events)
        contours_ids = [UNDEFINED_INDEX] * len(events)
        contours_vertices: List[List[Point]] = []
        depths: List[int] = []
        holes: List[List[int]] = []
        parents: List[int] = []
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
            if depths[contour_id] % 2 != 0:
                vertices = vertices[:1] + vertices[:0:-1]
            contours_vertices.append(vertices)
        result: List[Polygon] = []
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

    def to_event_end(self, event: Event) -> Point:
        return self.to_event_start(self._to_opposite_event(event))

    def to_event_start(self, event: Event) -> Point:
        return self._endpoints[event]

    __slots__ = (
        '_are_from_first_operand', '_are_from_result',
        '_other_have_interior_to_left', '_below_event_from_result',
        '_current_endpoint_first_event', '_current_endpoint_id', '_endpoints',
        '_events_queue_data', '_have_interior_to_left', '_opposites',
        '_overlap_kinds', '_segments_ids', '_starts_ids', '_sweep_line_data'
    )

    def __init__(self, segments_count: int) -> None:
        initial_events_count = 2 * segments_count
        self._are_from_first_operand: List[bool] = []
        self._are_from_result = [False] * segments_count
        self._other_have_interior_to_left = [False] * segments_count
        self._below_event_from_result = [UNDEFINED_EVENT] * segments_count
        self._current_endpoint_first_event = UNDEFINED_EVENT
        self._current_endpoint_id = 0
        self._endpoints: List[Point] = []
        self._events_queue_data: PriorityQueue[Event] = PriorityQueue(
                key=lambda event: EventsQueueKey(
                        event, self._is_from_first_operand_event(event),
                        self._endpoints, self._opposites
                )
        )
        self._have_interior_to_left = [True] * segments_count
        self._opposites: List[Event] = []
        self._overlap_kinds = [OverlapKind.NONE] * segments_count
        self._segments_ids = list(range(segments_count))
        self._starts_ids: List[int] = [UNDEFINED_INDEX] * initial_events_count
        self._sweep_line_data = red_black.set_(
                key=lambda event: SweepLineKey(
                        event, self._is_left_event_from_first_operand(event),
                        self._endpoints, self._opposites
                )
        )

    __repr__ = generate_repr(__init__)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self) -> Iterator[Event]:
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

    def _above(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.next(event)
        except ValueError:
            return None

    def _add(self, event: Event) -> None:
        assert is_left_event(event)
        self._sweep_line_data.add(event)

    def _below(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.prev(event)
        except ValueError:
            return None

    def _compute_left_event_fields(self,
                                   event: Event,
                                   below_event: Optional[Event]) -> None:
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
            are_internal: List[bool],
            depths: List[int],
            holes: List[List[int]],
            parents: List[int],
            are_from_in_to_out: Sequence[bool],
            contours_ids: Sequence[int],
            events_ids: Sequence[int],
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

    def _contour_events_to_vertices(self,
                                    events: Sequence[Event]) -> List[Point]:
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
        if (event_start_orientation is not Orientation.COLLINEAR
                and event_end_orientation is not Orientation.COLLINEAR):
            if event_start_orientation is not event_end_orientation:
                below_event_start_orientation = orient(event_start, event_end,
                                                       below_event_start)
                below_event_end_orientation = orient(event_start, event_end,
                                                     below_event_end)
                if (below_event_start_orientation is not Orientation.COLLINEAR
                        and (below_event_end_orientation
                             is not Orientation.COLLINEAR)):
                    if (below_event_start_orientation
                            is not below_event_end_orientation):
                        point = intersect_crossing_segments(
                                event_start, event_end, below_event_start,
                                below_event_end
                        )
                        assert event_start < point < event_end
                        assert below_event_start < point < below_event_end
                        self._divide_event_by_midpoint(below_event, point)
                        self._divide_event_by_midpoint(event,
                                                       point)
                elif below_event_start_orientation != Orientation.COLLINEAR:
                    if event_start < below_event_end < event_end:
                        point = below_event_end
                        self._divide_event_by_midpoint(event,
                                                       point)
                elif event_start < below_event_start < event_end:
                    point = below_event_start
                    self._divide_event_by_midpoint(event, point)
        elif event_end_orientation is not Orientation.COLLINEAR:
            if below_event_start < event_start < below_event_end:
                point = event_start
                self._divide_event_by_midpoint(below_event, point)
        elif event_start_orientation is not Orientation.COLLINEAR:
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

    def _divide(self, event: Event, mid_point: Point) -> Tuple[Event, Event]:
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
            mid_segment_event_start: Point,
            mid_segment_event_end: Point
    ) -> None:
        self._divide_event_by_midpoint(event, mid_segment_event_end)
        self._divide_event_by_midpoint(event, mid_segment_event_start)

    def _divide_event_by_midpoint(self, event: Event, point: Point) -> None:
        point_to_event_start_event, point_to_event_end_event = self._divide(
                event, point
        )
        self._push(point_to_event_start_event)
        self._push(point_to_event_end_event)

    def _divide_overlapping_events(self,
                                   min_start_event: Event,
                                   max_start_event: Event,
                                   max_start: Point,
                                   min_end: Point) -> None:
        self._divide_event_by_midpoint(max_start_event, min_end)
        self._divide_event_by_midpoint(min_start_event, max_start)

    def _events_to_connectivity(self, events: Sequence[Event]) -> List[int]:
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

    def _extend(self, segments: Iterable[Segment]) -> None:
        segment_id_offset = len(self._endpoints) // 2
        for segment_id, segment in enumerate(segments,
                                             start=segment_id_offset):
            start, end = segment.start, segment.end
            if start > end:
                start, end = end, start
                self._have_interior_to_left[segment_id] = False
            left_event = segment_id_to_left_event(segment_id)
            right_event = segment_id_to_right_event(segment_id)
            self._endpoints.append(start)
            self._endpoints.append(end)
            self._opposites.append(right_event)
            self._opposites.append(left_event)
            self._push(left_event)
            self._push(right_event)

    def _find(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            candidate = self._sweep_line_data.floor(event)
        except ValueError:
            return None
        else:
            return (candidate
                    if ((self.to_event_start(candidate)
                         == self.to_event_start(event))
                        and (self.to_event_end(candidate)
                             == self.to_event_end(event)))
                    else None)

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
        return self._are_from_first_operand[
            self._left_event_to_segment_id(event)
        ]

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
                                contour_events: Sequence[Event],
                                contour_id: int,
                                are_events_processed: List[bool],
                                are_from_in_to_out: List[bool],
                                contours_ids: List[int],
                                events_ids: Sequence[int]) -> None:
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
            events: Sequence[Event],
            events_ids: Sequence[int],
            connectivity: Sequence[int],
            are_events_processed: Sequence[bool],
            visited_endpoints_positions: List[int]
    ) -> List[Event]:
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


def _multisegmentals_to_segments_count(
        multisegmentals: Sequence[Multisegmental]
) -> int:
    return sum(multisegment.segments_count for multisegment in multisegmentals)


def _to_next_event_id(event_id: int,
                      are_events_processed: Sequence[bool],
                      connectivity: Sequence[int]) -> int:
    candidate = event_id
    while True:
        candidate = connectivity[candidate]
        if not are_events_processed[candidate]:
            return candidate
        elif candidate == event_id:
            return UNDEFINED_INDEX
