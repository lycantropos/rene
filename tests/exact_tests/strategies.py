from operator import attrgetter

from ground import hints
from hypothesis import strategies
from hypothesis_geometry import planar

from rene.exact import (
    Box,
    Contour,
    Empty,
    Multipolygon,
    Multisegment,
    Point,
    Polygon,
    Segment,
)

MAX_VALUE = 10**10
MIN_VALUE = -MAX_VALUE
integers = strategies.integers(MIN_VALUE, MAX_VALUE)
non_zero_integers = strategies.integers(MIN_VALUE, -1) | strategies.integers(
    1, MAX_VALUE
)
scalars_strategies = strategies.sampled_from(
    [
        integers,
        strategies.fractions(MIN_VALUE, MAX_VALUE, max_denominator=MAX_VALUE),
        strategies.floats(
            MIN_VALUE, MAX_VALUE, allow_infinity=False, allow_nan=False
        ),
    ]
)
empty_geometries = strategies.builds(Empty)


def to_box(raw_box: hints.Box) -> Box:
    return Box(raw_box.min_x, raw_box.max_x, raw_box.min_y, raw_box.max_y)


boxes = scalars_strategies.flatmap(planar.boxes).map(to_box)
boxes_limits = boxes.map(attrgetter('min_x', 'max_x', 'min_x', 'max_x'))


def to_point(raw_point: hints.Point) -> Point:
    return Point(raw_point.x, raw_point.y)


points = scalars_strategies.flatmap(planar.points).map(to_point)


def to_segment(raw_segment: hints.Segment) -> Segment:
    return Segment(to_point(raw_segment.start), to_point(raw_segment.end))


segments = scalars_strategies.flatmap(planar.segments).map(to_segment)
segments_endpoints = segments.map(attrgetter('start', 'end'))


def to_multisegment(raw_multisegment: hints.Multisegment) -> Multisegment:
    return Multisegment(
        [to_segment(segment) for segment in raw_multisegment.segments]
    )


multisegments = scalars_strategies.flatmap(planar.multisegments).map(
    to_multisegment
)
multisegments_segments = multisegments.map(attrgetter('segments'))
multisegments_segments |= multisegments_segments.map(
    list
) | multisegments_segments.map(tuple)


def to_contour(raw_contour: hints.Contour) -> Contour:
    return Contour([to_point(vertex) for vertex in raw_contour.vertices])


contours = scalars_strategies.flatmap(planar.contours).map(to_contour)
contours_vertices = contours.map(attrgetter('vertices'))
contours_vertices |= contours_vertices.map(list) | contours_vertices.map(tuple)


def to_polygon(raw_polygon: hints.Polygon) -> Polygon:
    return Polygon(
        to_contour(raw_polygon.border),
        [to_contour(hole) for hole in raw_polygon.holes],
    )


polygons = scalars_strategies.flatmap(planar.polygons).map(to_polygon)
polygons_components = polygons.map(attrgetter('border', 'holes'))
polygons_components |= polygons_components.map(
    lambda components: (components[0], list(components[1]))
) | polygons_components.map(
    lambda components: (components[0], tuple(components[1]))
)


def to_multipolygon(raw_multipolygon: hints.Multipolygon) -> Multipolygon:
    return Multipolygon(
        [to_polygon(polygon) for polygon in raw_multipolygon.polygons]
    )


multipolygons = scalars_strategies.flatmap(planar.multipolygons).map(
    to_multipolygon
)
multipolygons_polygons = multipolygons.map(attrgetter('polygons'))
multipolygons_polygons |= multipolygons_polygons.map(
    list
) | multipolygons_polygons.map(tuple)
