from hypothesis import given

from rene import Location
from rene.exact import (Polygon,
                        Point)
from tests.utils import (reverse_polygon_holes,
                         reverse_polygon_coordinates,
                         reverse_point_coordinates)
from . import strategies


@given(strategies.polygons, strategies.points)
def test_basic(polygon: Polygon, point: Point) -> None:
    result = polygon.locate(point)

    assert isinstance(result, Location)


@given(strategies.polygons)
def test_vertices(polygon: Polygon) -> None:
    assert ([vertex
             for vertex in polygon.border.vertices
             if polygon.locate(vertex) is not Location.BOUNDARY]
            == [])
    assert ([vertex
             for hole in polygon.holes
             for vertex in hole.vertices
             if polygon.locate(vertex) is not Location.BOUNDARY]
            == [])


@given(strategies.polygons, strategies.points)
def test_reversals(polygon: Polygon, point: Point) -> None:
    assert (polygon.locate(point)
            is reverse_polygon_holes(polygon).locate(point))
    assert (polygon.locate(point)
            is reverse_polygon_coordinates(polygon).locate(
                    reverse_point_coordinates(point)
            ))
