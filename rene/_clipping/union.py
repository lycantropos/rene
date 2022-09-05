from typing import List

from rene._utils import are_boxes_uncoupled
from rene.hints import Polygon
from .event import Event
from .operation import Operation


class Union(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_outside_left_event(event)
                or (not self._is_left_event_from_first_operand(event)
                    and self._is_common_region_boundary_left_event(event)))


def unite_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [first, second]
    operation = Union.from_multisegmentals(first, second)
    return operation.reduce_events(list(operation), type(first.border),
                                   type(first))
