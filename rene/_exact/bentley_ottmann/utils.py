from rene._exact.utils import cross_multiply
from rene.hints import Point


def intersect_crossing_segments(first_start: Point,
                                first_end: Point,
                                second_start: Point,
                                second_end: Point) -> Point:
    scale = (cross_multiply(first_start, second_start, second_start,
                            second_end)
             / cross_multiply(first_start, first_end, second_start,
                              second_end))
    return type(first_start)(
            first_start.x + (first_end.x - first_start.x) * scale,
            first_start.y + (first_end.y - first_start.y) * scale
    )
