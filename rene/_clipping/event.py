import sys
from typing import NewType

from rene._utils import (is_even,
                         is_odd)

Event = NewType('Event', int)

UNDEFINED_EVENT = Event(sys.maxsize)

is_left_event = is_even
is_right_event = is_odd


def left_event_to_position(event: Event) -> int:
    assert is_left_event(event)
    return event // 2


def segment_id_to_left_event(segment_id: int) -> Event:
    return Event(2 * segment_id)


def segment_id_to_right_event(segment_id: int) -> Event:
    return Event(2 * segment_id + 1)
