from __future__ import annotations

from rithm.fraction import Fraction

from rene import Orientation, hints
from rene._context import Context
from rene._utils import cross_multiply, to_sign

from .box import Box
from .contour import Contour
from .empty import Empty
from .multipolygon import Multipolygon
from .multisegment import Multisegment
from .point import Point
from .polygon import Polygon
from .segment import Segment
from .trapezoidation import Trapezoidation
from .triangulation import (
    ConstrainedDelaunayTriangulation,
    DelaunayTriangulation,
)


def orient(
    vertex: hints.Point[Fraction],
    first_ray_point: hints.Point[Fraction],
    second_ray_point: hints.Point[Fraction],
    /,
) -> Orientation:
    raw = to_sign(cross_multiply(vertex, first_ray_point, vertex, second_ray_point))
    return (
        Orientation.COLLINEAR
        if raw == 0
        else (Orientation.COUNTERCLOCKWISE if raw > 0 else Orientation.CLOCKWISE)
    )


def to_segments_intersection(
    first_start: hints.Point[Fraction],
    first_end: hints.Point[Fraction],
    second_start: hints.Point[Fraction],
    second_end: hints.Point[Fraction],
    /,
) -> hints.Point[Fraction]:
    scale = to_segments_intersection_scale(
        first_start, first_end, second_start, second_end
    )
    return Point(
        first_start.x + (first_end.x - first_start.x) * scale,
        first_start.y + (first_end.y - first_start.y) * scale,
    )


def to_segments_intersection_scale(
    first_start: hints.Point[hints.Scalar],
    first_end: hints.Point[hints.Scalar],
    second_start: hints.Point[hints.Scalar],
    second_end: hints.Point[hints.Scalar],
    /,
) -> hints.Scalar:
    return cross_multiply(
        first_start, second_start, second_start, second_end
    ) / cross_multiply(first_start, first_end, second_start, second_end)


_context = Context(
    box_cls=Box,
    contour_cls=Contour,
    empty_cls=Empty,
    multipolygon_cls=Multipolygon,
    multisegment_cls=Multisegment,
    orienteer=orient,
    point_cls=Point,
    polygon_cls=Polygon,
    segment_cls=Segment,
    segments_intersection_scale=to_segments_intersection_scale,
    segments_intersector=to_segments_intersection,
)
ConstrainedDelaunayTriangulation._context = _context
Contour._context = _context
DelaunayTriangulation._context = _context
Empty._context = _context
Multipolygon._context = _context
Multisegment._context = _context
Polygon._context = _context
Segment._context = _context
Trapezoidation._context = _context
del _context
