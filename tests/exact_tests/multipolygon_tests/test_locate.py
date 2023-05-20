from hypothesis import given

from rene import Location
from rene.exact import (Multipolygon,
                        Point)
from tests.utils import (reverse_multipolygon,
                         reverse_multipolygon_coordinates,
                         reverse_point_coordinates)
from . import strategies


@given(strategies.multipolygons, strategies.points)
def test_basic(multipolygon: Multipolygon, point: Point) -> None:
    result = multipolygon.locate(point)

    assert isinstance(result, Location)


@given(strategies.multipolygons)
def test_vertices(multipolygon: Multipolygon) -> None:
    assert ([vertex
             for polygon in multipolygon.polygons
             for vertex in polygon.border.vertices
             if multipolygon.locate(vertex) is not Location.BOUNDARY]
            == [])
    assert ([vertex
             for polygon in multipolygon.polygons
             for hole in polygon.holes
             for vertex in hole.vertices
             if multipolygon.locate(vertex) is not Location.BOUNDARY]
            == [])


@given(strategies.multipolygons, strategies.points)
def test_reversals(multipolygon: Multipolygon, point: Point) -> None:
    assert (multipolygon.locate(point)
            is reverse_multipolygon(multipolygon).locate(point))
    assert (multipolygon.locate(point)
            is reverse_multipolygon_coordinates(multipolygon).locate(
                    reverse_point_coordinates(point)
            ))
