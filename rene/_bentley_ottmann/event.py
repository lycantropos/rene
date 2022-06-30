from typing import NewType

Event = NewType('Event', int)


def is_left_event(event: Event) -> bool:
    return event % 2 == 0


def segment_id_to_left_event(segment_id: int) -> Event:
    return Event(2 * segment_id)


def segment_id_to_right_event(segment_id: int) -> Event:
    return Event(2 * segment_id + 1)
