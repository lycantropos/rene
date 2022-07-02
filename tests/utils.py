from functools import partial
from typing import (Callable,
                    Iterable,
                    List,
                    Sequence,
                    TypeVar)

from rene import MIN_CONTOUR_VERTICES_COUNT
from rene._rene import (Location,
                        Orientation)
from rene._utils import (deduplicate,
                         locate_point_in_point_point_point_circle,
                         orient)
from rene.hints import (Contour,
                        Multisegment,
                        Point,
                        Polygon,
                        Segment)

_T1 = TypeVar('_T1')
_T2 = TypeVar('_T2')


def apply(function: Callable[..., _T2], args: Iterable[_T1]) -> _T2:
    return function(*args)


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool) -> bool:
    return not antecedent or consequent


def is_contour_triangular(contour: Contour) -> bool:
    return len(contour.vertices) == MIN_CONTOUR_VERTICES_COUNT


def is_point_inside_circumcircle(point: Point,
                                 first_vertex: Point,
                                 second_vertex: Point,
                                 third_vertex: Point) -> bool:
    return locate_point_in_point_point_point_circle(
            point, first_vertex, second_vertex, third_vertex
    ) is Location.INTERIOR


def pack(function: Callable[..., _T2]) -> Callable[[Iterable[_T1]], _T2]:
    return partial(apply, function)


def reverse_contour(contour: Contour) -> Contour:
    return type(contour)(contour.vertices[::-1])


def reverse_each_polygon_hole(polygon: Polygon) -> Polygon:
    return type(polygon)(polygon.border,
                         [reverse_contour(hole) for hole in polygon.holes])


def reverse_multisegment(multisegment: Multisegment) -> Multisegment:
    return type(multisegment)(multisegment.segments[::-1])


def reverse_polygon_border(polygon: Polygon) -> Polygon:
    return type(polygon)(reverse_contour(polygon.border), polygon.holes)


def reverse_polygon_holes(polygon: Polygon) -> Polygon:
    return type(polygon)(polygon.border, polygon.holes[::-1])


def reverse_segment_endpoints(segment: Segment) -> Segment:
    return type(segment)(segment.end, segment.start)


def rotate_contour(contour: Contour, offset: int) -> Contour:
    return type(contour)(rotate_sequence(contour.vertices, offset))


def rotate_each_polygon_hole(polygon: Polygon, offset: int) -> Polygon:
    return type(polygon)(polygon.border,
                         [rotate_contour(hole, offset)
                          for hole in polygon.holes])


def rotate_multisegment(multisegment: Multisegment, offset: int
                        ) -> Multisegment:
    return type(multisegment)(rotate_sequence(multisegment.segments, offset))


def rotate_polygon_border(polygon: Polygon, offset: int) -> Polygon:
    return type(polygon)(rotate_contour(polygon.border, offset), polygon.holes)


def rotate_polygon_holes(polygon: Polygon, offset: int) -> Polygon:
    return type(polygon)(polygon.border,
                         rotate_sequence(polygon.holes, offset))


def rotate_sequence(sequence: Sequence[_T1], offset: int) -> Sequence[_T1]:
    if not sequence:
        return sequence
    offset = (offset % len(sequence)) - len(sequence) * (offset < 0)
    return sequence[-offset:] + sequence[:-offset]


to_distinct = dict.fromkeys


def to_max_convex_hull(points: Sequence[Point]) -> List[Point]:
    points = deduplicate(sorted(points))
    lower, upper = _to_sub_hull(points), _to_sub_hull(reversed(points))
    return lower[:-1] + upper[:-1] or points


def _to_sub_hull(points: Iterable[Point]) -> List[Point]:
    result = []
    for point in points:
        while len(result) >= 2:
            if orient(result[-2], result[-1], point) is Orientation.CLOCKWISE:
                del result[-1]
            else:
                break
        result.append(point)
    return result
