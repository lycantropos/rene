from typing import List

from rene.hints import Polygon
from .event import Event
from .operation import Operation


class Intersection(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_inside_left_event(event)
                or not self._is_left_event_from_first_operand(event)
                and self._is_common_region_boundary_left_event(event))


def intersect_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if (first_bounding_box.touches(second_bounding_box)
            or first_bounding_box.touches(second_bounding_box)):
        return []
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = Intersection.from_multisegmentals(first, second)
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))
