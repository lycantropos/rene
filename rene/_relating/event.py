import sys
from typing import NewType

from rene._utils import (is_even,
                         is_odd)

Event = NewType('Event', int)

UNDEFINED_EVENT = Event(sys.maxsize)

is_left_event = is_even
is_right_event = is_odd


def left_event_to_position(event: Event, /) -> int:
    assert is_left_event(event)
    return event // 2
