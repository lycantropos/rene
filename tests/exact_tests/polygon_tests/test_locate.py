from hypothesis import given

from rene.enums import Location
from rene.exact import Point, Polygon
from tests.utils import reverse_polygon_holes

from . import strategies


@given(strategies.polygons)
def test_vertices(polygon: Polygon) -> None:
    assert [
        vertex
        for vertex in polygon.border.vertices
        if polygon.locate(vertex) is not Location.BOUNDARY
    ] == []
    assert [
        vertex
        for hole in polygon.holes
        for vertex in hole.vertices
        if polygon.locate(vertex) is not Location.BOUNDARY
    ] == []


@given(strategies.polygons, strategies.points)
def test_reversals(polygon: Polygon, point: Point) -> None:
    assert polygon.locate(point) is reverse_polygon_holes(polygon).locate(
        point
    )
