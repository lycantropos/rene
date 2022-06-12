from rene._exact import Point
from rene._exact.utils import cross_multiply
from rene._rene import (Relation)


def classify_overlap(test_start: Point,
                     test_end: Point,
                     goal_start: Point,
                     goal_end: Point) -> Relation:
    assert test_start < test_end
    assert goal_start < goal_end
    if test_start == goal_start:
        return (Relation.COMPONENT
                if test_end < goal_end
                else (Relation.COMPOSITE
                      if goal_end < test_end
                      else Relation.EQUAL))
    elif test_end == goal_end:
        return (Relation.COMPOSITE
                if test_start < goal_start
                else Relation.COMPONENT)
    elif goal_start < test_start < goal_end:
        return (Relation.COMPONENT
                if test_end < goal_end
                else Relation.OVERLAP)
    else:
        assert test_start < goal_start < test_end
        return (Relation.COMPOSITE
                if goal_end < test_end
                else Relation.OVERLAP)


def intersect_crossing_segments(first_start: Point,
                                first_end: Point,
                                second_start: Point,
                                second_end: Point) -> Point:
    scale = (cross_multiply(first_start, second_start, second_start,
                            second_end)
             / cross_multiply(first_start, first_end, second_start,
                              second_end))
    return Point(first_start.x + (first_end.x - first_start.x) * scale,
                 first_start.y + (first_end.y - first_start.y) * scale)
