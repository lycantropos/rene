from hypothesis import given

from rene import Location
from rene.exact import Multipolygon, Point
from tests.utils import reverse_multipolygon_polygons

from . import strategies


@given(strategies.multipolygons)
def test_vertices(multipolygon: Multipolygon) -> None:
    assert [
        vertex
        for polygon in multipolygon.polygons
        for vertex in polygon.border.vertices
        if multipolygon.locate(vertex) is not Location.BOUNDARY
    ] == []
    assert [
        vertex
        for polygon in multipolygon.polygons
        for hole in polygon.holes
        for vertex in hole.vertices
        if multipolygon.locate(vertex) is not Location.BOUNDARY
    ] == []


@given(strategies.multipolygons, strategies.points)
def test_reversals(multipolygon: Multipolygon, point: Point) -> None:
    assert multipolygon.locate(point) is reverse_multipolygon_polygons(
        multipolygon
    ).locate(point)
