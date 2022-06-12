from functools import partial
from typing import (Iterator, List, Optional, Sequence,
                    Tuple)

from dendroid import red_black
from prioq.base import PriorityQueue
from reprit.base import generate_repr

from rene import Orientation
from rene._exact import (Point,
                         Segment)
from rene._exact.utils import orient, to_sorted_pair
from .event import (Event,
                    is_left_event)
from .events_queue_key import EventsQueueKey
from .sweep_line_key import SweepLineKey
from .utils import intersect_crossing_segments


class EventsRegistry:
    @classmethod
    def from_segments(cls, segments: Sequence[Segment], *, unique: bool
                      ) -> 'EventsRegistry':
        result = cls(unique=unique)
        for index, segment in enumerate(segments):
            left_event = Event(2 * index)
            right_event = Event(2 * index + 1)
            start, end = to_sorted_pair(segment.start, segment.end)
            result._endpoints.append(start)
            result._endpoints.append(end)
            result._opposites.append(right_event)
            result._opposites.append(left_event)
            result.push(left_event)
            result.push(right_event)
        result._min_collinear_segments_ids = list(range(len(segments)))
        result._segments_ids = list(range(len(segments)))
        return result

    @property
    def unique(self) -> bool:
        return self._unique

    __slots__ = ('_endpoints', '_events_queue_data',
                 '_min_collinear_segments_ids', '_opposites', '_segments_ids',
                 '_sweep_line_data', '_unique')

    def __init__(self, *, unique: bool) -> None:
        self._unique = unique
        self._opposites: List[Event] = []
        self._endpoints: List[Point] = []
        self._segments_ids: List[int] = []
        self._min_collinear_segments_ids: List[int] = []
        self._events_queue_data: PriorityQueue[Event] = PriorityQueue(
                key=partial(EventsQueueKey, self._endpoints, self._opposites)
        )
        self._sweep_line_data = red_black.set_(key=partial(SweepLineKey,
                                                           self._endpoints,
                                                           self._opposites))

    def add(self, event: Event) -> None:
        assert is_left_event(event)
        self._sweep_line_data.add(event)

    def find(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            candidate = self._sweep_line_data.floor(event)
        except ValueError:
            return None
        else:
            return (candidate
                    if (self._endpoints[candidate] == self._endpoints[event]
                        and (self._endpoints[self._opposites[candidate]]
                             == self._endpoints[self._opposites[event]]))
                    else None)

    def remove(self, event: Event) -> None:
        assert is_left_event(event)
        self._sweep_line_data.remove(event)

    def above(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.next(event)
        except ValueError:
            return None

    def below(self, event: Event) -> Optional[Event]:
        assert is_left_event(event)
        try:
            return self._sweep_line_data.prev(event)
        except ValueError:
            return None

    __repr__ = generate_repr(__init__)

    def __bool__(self) -> bool:
        return bool(self._events_queue_data)

    def __iter__(self) -> Iterator[Event]:
        while self:
            event = self.pop()
            if is_left_event(event):
                equal_segment_event = self.find(event)
                if equal_segment_event is None:
                    self.add(event)
                    below_event = self.below(event)
                    if below_event is not None:
                        self.detect_intersection(below_event, event)
                    above_event = self.above(event)
                    if above_event is not None:
                        self.detect_intersection(event, above_event)
                    yield event
                else:
                    self.merge_equal_segment_events(
                            equal_segment_event,
                            event)
                    if not self.unique:
                        yield event
            else:
                event = self.get_opposite_event(event)
                equal_segment_event = self.find(event)
                if equal_segment_event is not None:
                    above_event, below_event = (
                        self.above(equal_segment_event),
                        self.below(equal_segment_event)
                    )
                    self.remove(equal_segment_event)
                    if below_event is not None and above_event is not None:
                        self.detect_intersection(below_event,
                                                 above_event)
                    if event != equal_segment_event:
                        self.merge_equal_segment_events(
                                equal_segment_event, event
                        )
                elif not self.unique:
                    yield event

    def detect_intersection(self, below_event: Event, event: Event) -> None:
        event_start = self.get_event_start(event)
        event_end = self.get_event_end(event)
        below_event_start = self.get_event_start(below_event)
        below_event_end = self.get_event_end(below_event)
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
                        self.divide_event_by_midpoint(below_event, point)
                        self.divide_event_by_midpoint_checking_above(event,
                                                                     point)
                elif below_event_start_orientation != Orientation.COLLINEAR:
                    if event_start < below_event_end < event_end:
                        point = below_event_end
                        self.divide_event_by_midpoint_checking_above(event,
                                                                     point)
                elif event_start < below_event_start < event_end:
                    point = below_event_start
                    self.divide_event_by_midpoint_checking_above(event, point)
        elif event_end_orientation is not Orientation.COLLINEAR:
            if below_event_start < event_start < below_event_end:
                point = event_start
                self.divide_event_by_midpoint(below_event, point)
        elif event_start_orientation is not Orientation.COLLINEAR:
            if below_event_start < event_end < below_event_end:
                point = event_end
                self.divide_event_by_midpoint(below_event, point)
        elif event_start == below_event_start:
            assert event_end != below_event_end
            max_end_event, min_end_event = ((below_event, event)
                                            if event_end < below_event_end
                                            else (event, below_event))
            self.remove(max_end_event)
            min_end = self.get_event_end(min_end_event)
            _, min_end_max_end_event = self.divide(max_end_event, min_end)
            self.push(min_end_max_end_event)
            self.merge_equal_segment_events(event, below_event)
        elif event_end == below_event_end:
            max_start_event, min_start_event = (
                (below_event, event)
                if event_start < below_event_start
                else (event, below_event)
            )
            max_start = self.get_event_start(max_start_event)
            (
                max_start_to_min_start_event, max_start_to_end_event
            ) = self.divide(min_start_event, max_start)
            self.push(max_start_to_min_start_event)
            self.merge_equal_segment_events(max_start_event,
                                            max_start_to_end_event)
        elif below_event_start < event_start < below_event_end:
            if event_end < below_event_end:
                self.divide_event_by_mid_segment_event_endpoints(
                        below_event, event, event_start, event_end
                )
            else:
                max_start, min_end = event_start, below_event_end
                self.divide_overlapping_events(below_event, event, max_start,
                                               min_end)
        elif event_start < below_event_start < event_end:
            if below_event_end < event_end:
                self.divide_event_by_mid_segment_event_endpoints(
                        event, below_event, below_event_start, below_event_end
                )
            else:
                max_start, min_end = below_event_start, event_end
                self.divide_overlapping_events(event, below_event, max_start,
                                               min_end)

    def divide(self, event: Event, mid_point: Point) -> Tuple[Event, Event]:
        assert is_left_event(event)
        opposite_event = self.get_opposite_event(event)
        mid_point_to_event_end_event = Event(len(self._endpoints))
        self._segments_ids.append(self.to_left_event_segment_id(event))
        self._endpoints.append(mid_point)
        self._opposites.append(opposite_event)
        self._opposites[opposite_event] = mid_point_to_event_end_event
        mid_point_to_event_start_event = Event(len(self._endpoints))
        self._endpoints.append(mid_point)
        self._opposites.append(event)
        self._opposites[event] = mid_point_to_event_start_event
        return mid_point_to_event_start_event, mid_point_to_event_end_event

    def divide_event_by_mid_segment_event_endpoints(
            self,
            event: Event,
            mid_segment_event: Event,
            mid_segment_event_start: Point,
            mid_segment_event_end: Point
    ) -> None:
        self.divide_event_by_midpoint(event, mid_segment_event_end)
        (
            mid_segment_event_start_to_event_start_event,
            min_segment_event_start_to_min_segment_event_end_event,
        ) = self.divide(event, mid_segment_event_start)
        self.push(mid_segment_event_start_to_event_start_event)
        self.merge_equal_segment_events(
                mid_segment_event,
                min_segment_event_start_to_min_segment_event_end_event,
        )

    def divide_event_by_midpoint(self, event: Event, point: Point) -> None:
        point_to_event_start_event, point_to_event_end_event = self.divide(
                event, point
        )
        self.push(point_to_event_start_event)
        self.push(point_to_event_end_event)

    def divide_event_by_midpoint_checking_above(self,
                                                event: Event,
                                                point: Point) -> None:
        above_event = self.above(event)
        if above_event is not None:
            if ((self.get_event_start(above_event)
                 == self.get_event_start(event))
                    and self.get_event_end(above_event) == point):
                self.remove(above_event)
                self.divide_event_by_midpoint(event, point)
                self.merge_equal_segment_events(event, above_event)
                return
        self.divide_event_by_midpoint(event, point)

    def divide_overlapping_events(self,
                                  min_start_event: Event,
                                  max_start_event: Event,
                                  max_start: Point,
                                  min_end: Point) -> None:
        self.divide_event_by_midpoint(max_start_event, min_end)
        (
            max_start_to_min_start_event, max_start_to_min_end_event
        ) = self.divide(min_start_event, max_start)
        self.push(max_start_to_min_start_event)
        self.merge_equal_segment_events(max_start_event,
                                        max_start_to_min_end_event)

    def merge_equal_segment_events(self, first: Event, second: Event) -> None:
        first_segment_id = self.to_left_event_segment_id(first)
        second_segment_id = self.to_left_event_segment_id(second)
        first_min_collinear_segment_id = (
            self._min_collinear_segments_ids[first_segment_id]
        )
        second_min_collinear_segment_id = (
            self._min_collinear_segments_ids[second_segment_id]
        )
        min_collinear_segment_id = min(first_min_collinear_segment_id,
                                       second_min_collinear_segment_id)
        self._min_collinear_segments_ids[first_segment_id] = (
            min_collinear_segment_id
        )
        self._min_collinear_segments_ids[second_segment_id] = (
            min_collinear_segment_id
        )
        self._min_collinear_segments_ids[first_min_collinear_segment_id] = (
            min_collinear_segment_id
        )
        self._min_collinear_segments_ids[second_min_collinear_segment_id] = (
            min_collinear_segment_id
        )

    def get_left_event_segment_id(self, event: Event) -> int:
        assert is_left_event(event)
        return self._segments_ids[event // 2]

    def get_event_start(self, event: Event) -> Point:
        return self._endpoints[event]

    def get_event_end(self, event: Event) -> Point:
        return self.get_event_start(self.get_opposite_event(event))

    def get_opposite_event(self, event: Event) -> Event:
        return self._opposites[event]

    def pop(self) -> Event:
        return self._events_queue_data.pop()

    def push(self, event: Event) -> None:
        self._events_queue_data.push(event)

    def to_left_event_segment_id(self, event: Event) -> int:
        assert is_left_event(event)
        return self._segments_ids[event // 2]

    def get_event_segment_id(self, event: Event) -> int:
        return self.get_left_event_segment_id(event
                                              if is_left_event(event)
                                              else self._opposites[event])

    def are_collinear(self,
                      first_segment_id: int,
                      second_segment_id: int) -> bool:
        return (self.to_min_collinear_segment_id(first_segment_id)
                == self.to_min_collinear_segment_id(second_segment_id))

    def to_min_collinear_segment_id(self, segment_id: int) -> int:
        candidate = segment_id
        iterations_count = 0
        while self._min_collinear_segments_ids[candidate] != candidate:
            candidate = self._min_collinear_segments_ids[candidate]
            iterations_count += 1
        assert iterations_count < 3
        return candidate

    def get_segment_end(self, segment_id: int) -> Point:
        return self._endpoints[Event(2 * segment_id + 1)]

    def get_segment_start(self, segment_id: int) -> Point:
        return self._endpoints[Event(2 * segment_id)]
