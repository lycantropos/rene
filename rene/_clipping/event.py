import sys
from typing import NewType

Event = NewType('Event', int)

UNDEFINED_EVENT = Event(sys.maxsize)


def is_left_event(event: Event) -> bool:
    return event % 2 == 0


def is_right_event(event: Event) -> bool:
    return event % 2 != 0


def left_event_to_position(event: Event) -> int:
    assert is_left_event(event)
    return event // 2


def segment_id_to_left_event(segment_id: int) -> Event:
    return Event(2 * segment_id)


def segment_id_to_right_event(segment_id: int) -> Event:
    return Event(2 * segment_id + 1)
