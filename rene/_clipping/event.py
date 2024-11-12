import sys
from typing import NewType

from rene._utils import is_even, is_odd

Event = NewType('Event', int)

UNDEFINED_EVENT = Event(sys.maxsize)

is_event_left = is_even
is_event_right = is_odd


def left_event_to_position(event: Event, /) -> int:
    assert is_event_left(event)
    return event // 2
