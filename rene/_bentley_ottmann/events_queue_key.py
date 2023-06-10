from __future__ import annotations

import typing as t

import typing_extensions as te
from reprit.base import generate_repr

from rene._hints import Map
from rene.hints import (Point,
                        Scalar)
from .event import (Event,
                    is_left_event)


class EventsQueueKey(t.Generic[Scalar]):
    endpoints: Map[Event, Point[Scalar]]
    opposites: Map[Event, Event]
    event: Event

    __slots__ = 'endpoints', 'event', 'opposites'

    def __new__(cls,
                endpoints: Map[Event, Point[Scalar]],
                opposites: Map[Event, Event],
                event: Event,
                /) -> te.Self:
        self = super().__new__(cls)
        self.endpoints, self.event, self.opposites = (endpoints, event,
                                                      opposites)
        return self

    __repr__ = generate_repr(__new__)

    def __lt__(self, other: te.Self, /) -> bool:
        """
        Checks if the event should be processed before the other.
        """
        event, other_event = self.event, other.event
        event_start, other_event_start = (self.endpoints[event],
                                          self.endpoints[other_event])
        start_x, start_y = event_start.x, event_start.y
        other_start_x, other_start_y = other_event_start.x, other_event_start.y
        if start_x != other_start_x:
            # different x-coordinate,
            # the event with lower x-coordinate is processed first
            return start_x < other_start_x
        elif start_y != other_start_y:
            # different starts, but same x-coordinate,
            # the event with lower y-coordinate is processed first
            return start_y < other_start_y
        elif is_left_event(event) is not is_left_event(other_event):
            # same start, but one is a left endpoint
            # and the other is a right endpoint,
            # the right endpoint is processed first
            return not is_left_event(event)
        else:
            # same start,
            # both events are left endpoints or both are right endpoints
            return (self.endpoints[self.opposites[event]]
                    < self.endpoints[self.opposites[other_event]])
