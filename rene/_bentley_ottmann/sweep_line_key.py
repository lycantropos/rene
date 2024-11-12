from typing import Generic

from typing_extensions import Self

from rene import Orientation, hints
from rene._hints import Map, Orienteer

from .event import Event


class SweepLineKey(Generic[hints.Scalar]):
    endpoints: Map[Event, hints.Point[hints.Scalar]]
    opposites: Map[Event, Event]
    event: Event
    _orienteer: Orienteer[hints.Scalar]

    __slots__ = 'endpoints', 'event', 'opposites', '_orienteer'

    def __new__(
        cls,
        endpoints: Map[Event, hints.Point[hints.Scalar]],
        opposites: Map[Event, Event],
        event: Event,
        orienteer: Orienteer[hints.Scalar],
        /,
    ) -> Self:
        self = super().__new__(cls)
        self.endpoints, self.event, self.opposites, self._orienteer = (
            endpoints,
            event,
            opposites,
            orienteer,
        )
        return self

    def __lt__(self, other: Self, /) -> bool:
        """
        Checks if the segment (or at least the point) associated with event
        is lower than other's.
        """
        assert self.endpoints is other.endpoints
        assert self.opposites is other.opposites
        event, other_event = self.event, other.event
        start, other_start = self.endpoints[event], self.endpoints[other_event]
        end, other_end = (
            self.endpoints[self.opposites[event]],
            self.endpoints[self.opposites[other_event]],
        )
        other_start_orientation = self._orienteer(start, end, other_start)
        other_end_orientation = self._orienteer(start, end, other_end)
        if other_start_orientation is other_end_orientation:
            start_x, start_y = start.x, start.y
            other_start_x, other_start_y = other_start.x, other_start.y
            if other_start_orientation is not Orientation.COLLINEAR:
                # other segment fully lies on one side
                return other_start_orientation is Orientation.COUNTERCLOCKWISE
            # segments are collinear
            elif start_y == other_start_y:
                end_x, end_y = end.x, end.y
                other_end_x, other_end_y = other_end.x, other_end.y
                if start_x != other_start_x:
                    return start_x < other_start_x
                # segments have same start
                elif end_y != other_end_y:
                    return end_y < other_end_y
                else:
                    # segments are horizontal
                    return end_x < other_end_x
            else:
                return start_y < other_start_y
        start_orientation = self._orienteer(other_start, other_end, start)
        end_orientation = self._orienteer(other_start, other_end, end)
        if start_orientation is end_orientation:
            return start_orientation is Orientation.CLOCKWISE
        elif other_start_orientation is Orientation.COLLINEAR:
            return other_end_orientation is Orientation.COUNTERCLOCKWISE
        elif start_orientation is Orientation.COLLINEAR:
            return end_orientation is Orientation.CLOCKWISE
        elif end_orientation is Orientation.COLLINEAR:
            return start_orientation is Orientation.CLOCKWISE
        else:
            return other_start_orientation is Orientation.COUNTERCLOCKWISE
