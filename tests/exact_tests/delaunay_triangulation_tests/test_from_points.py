from typing import Sequence

from hypothesis import given

from rene import MIN_CONTOUR_VERTICES_COUNT
from rene._exact import orient
from rene.exact import (Contour,
                        DelaunayTriangulation,
                        Point)
from tests.utils import (is_contour_triangular,
                         is_point_inside_circumcircle,
                         to_convex_hull,
                         to_distinct,
                         to_max_convex_hull)
from . import strategies


@given(strategies.points_lists)
def test_basic(points: Sequence[Point]) -> None:
    result = DelaunayTriangulation.from_points(points)

    assert isinstance(result, DelaunayTriangulation)


@given(strategies.points_lists)
def test_border(points: Sequence[Point]) -> None:
    result = DelaunayTriangulation.from_points(points)

    convex_hull = to_convex_hull(points, orient)
    assert (len(convex_hull) < MIN_CONTOUR_VERTICES_COUNT
            or result.border == Contour(convex_hull))


@given(strategies.points_lists)
def test_triangles(points: Sequence[Point]) -> None:
    result = DelaunayTriangulation.from_points(points)

    triangles = result.triangles
    assert len(triangles) <= max(2 * (len(to_distinct(points)) - 1)
                                 - len(to_max_convex_hull(points, orient)),
                                 0)
    assert all(is_contour_triangular(triangle) for triangle in triangles)


@given(strategies.points_lists)
def test_delaunay_criterion(points: Sequence[Point]) -> None:
    result = DelaunayTriangulation.from_points(points)

    assert all(not any(is_point_inside_circumcircle(point, *triangle.vertices)
                       for triangle in result.triangles)
               for point in points)


@given(strategies.points)
def test_base_case(point: Point) -> None:
    result = DelaunayTriangulation.from_points([point])

    triangles = result.triangles
    assert len(triangles) == 0


@given(strategies.two_or_more_points_lists)
def test_step(points: Sequence[Point]) -> None:
    rest_points, last_point = points[:-1], points[-1]

    result = DelaunayTriangulation.from_points(rest_points)
    next_result = DelaunayTriangulation.from_points(points)

    triangles = result.triangles
    next_triangles = next_result.triangles
    assert len(triangles) <= len(next_triangles) + 2
    assert all(triangle not in next_triangles
               for triangle in triangles
               if is_point_inside_circumcircle(last_point, *triangle.vertices))
