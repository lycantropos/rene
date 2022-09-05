from typing import List

from rene.hints import Polygon
from .event import Event
from .operation import Operation


class Difference(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_outside_left_event(event)
                if self._is_left_event_from_first_operand(event)
                else (self._is_inside_left_event(event)
                      or self._is_common_polyline_component_left_event(event)))


def subtract_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if (first_bounding_box.touches(second_bounding_box)
            or first_bounding_box.touches(second_bounding_box)):
        return [first]
    operation = Difference.from_multisegmentals(first, second)
    return operation.reduce_events(list(operation), type(first.border),
                                   type(first))
