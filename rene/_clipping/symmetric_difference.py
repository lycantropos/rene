from typing import List

from rene.hints import Polygon
from .event import Event
from .operation import Operation


class SymmetricDifference(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return not self._is_overlap_left_event(event)


def symmetric_subtract_polygons(first: Polygon,
                                second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if (first_bounding_box.touches(second_bounding_box)
            or first_bounding_box.touches(second_bounding_box)):
        return [first, second]
    operation = SymmetricDifference.from_polygons(first, second)
    return operation.reduce_events(list(operation))
