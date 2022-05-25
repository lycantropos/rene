from functools import partial
from typing import (Callable,
                    Iterable,
                    Sequence,
                    TypeVar)

from rene.exact import (Contour,
                        Polygon)

_T1 = TypeVar('_T1')
_T2 = TypeVar('_T2')


def apply(function: Callable[..., _T2], args: Iterable[_T1]) -> _T2:
    return function(*args)


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool) -> bool:
    return not antecedent or consequent


def pack(function: Callable[..., _T2]) -> Callable[[Iterable[_T1]], _T2]:
    return partial(apply, function)


def reverse_contour(contour: Contour) -> Contour:
    return type(contour)(contour.vertices[::-1])


def reverse_each_polygon_hole(polygon: Polygon) -> Polygon:
    return type(polygon)(polygon.border,
                         [reverse_contour(hole) for hole in polygon.holes])


def reverse_polygon_border(polygon: Polygon) -> Polygon:
    return type(polygon)(reverse_contour(polygon.border), polygon.holes)


def reverse_polygon_holes(polygon: Polygon) -> Polygon:
    return type(polygon)(polygon.border, polygon.holes[::-1])


def rotate_contour(contour: Contour, offset: int) -> Contour:
    return type(contour)(rotate_sequence(contour.vertices, offset))


def rotate_each_polygon_hole(polygon: Polygon, offset: int) -> Polygon:
    return type(polygon)(polygon.border,
                         [rotate_contour(hole, offset)
                          for hole in polygon.holes])


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
