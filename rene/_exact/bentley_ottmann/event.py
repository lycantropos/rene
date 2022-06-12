from typing import NewType

Event = NewType('Event', int)


def is_left_event(event: Event) -> bool:
    return event % 2 == 0
