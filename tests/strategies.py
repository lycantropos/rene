from operator import attrgetter

from ground import hints
from hypothesis import strategies
from hypothesis_geometry import planar

from rene.exact import (Contour,
                        Multisegment,
                        Point,
                        Polygon,
                        Segment)

MAX_VALUE = 10 ** 10
MIN_VALUE = -MAX_VALUE
integers = strategies.integers(MIN_VALUE, MAX_VALUE)
non_zero_integers = (strategies.integers(MIN_VALUE, -1)
                     | strategies.integers(1, MAX_VALUE))
scalars_strategies = strategies.sampled_from([
    integers,
    strategies.fractions(MIN_VALUE, MAX_VALUE,
                         max_denominator=MAX_VALUE),
    strategies.floats(MIN_VALUE, MAX_VALUE,
                      allow_infinity=False,
                      allow_nan=False)
])


def to_point(raw_point: hints.Point) -> Point:
    return Point(raw_point.x, raw_point.y)


points = scalars_strategies.flatmap(planar.points).map(to_point)


def to_segment(raw_segment: hints.Segment) -> Segment:
    return Segment(to_point(raw_segment.start), to_point(raw_segment.end))


segments = scalars_strategies.flatmap(planar.segments).map(to_segment)


def to_multisegment(raw_contour: hints.Multisegment) -> Multisegment:
    return Multisegment([to_segment(segment)
                         for segment in raw_contour.segments])


multisegments = (scalars_strategies
                 .flatmap(planar.multisegments)
                 .map(to_multisegment))
multisegments_segments = multisegments.map(attrgetter('segments'))


def to_contour(raw_contour: hints.Contour) -> Contour:
    return Contour([to_point(vertex) for vertex in raw_contour.vertices])


contours = scalars_strategies.flatmap(planar.contours).map(to_contour)
contours_vertices = contours.map(attrgetter('vertices'))
polygons = scalars_strategies.flatmap(planar.polygons).map(
        lambda raw_polygon: Polygon(to_contour(raw_polygon.border),
                                    [to_contour(hole)
                                     for hole in raw_polygon.holes])
)
polygons_components = polygons.map(attrgetter('border', 'holes'))
