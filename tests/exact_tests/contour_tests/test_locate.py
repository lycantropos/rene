from hypothesis import given

from rene import Location
from rene.exact import (Contour,
                        Point)
from . import strategies


@given(strategies.contours, strategies.points)
def test_basic(contour: Contour, point: Point) -> None:
    result = contour.locate(point)

    assert isinstance(result, Location)


@given(strategies.contours)
def test_vertices(contour: Contour) -> None:
    assert [vertex
            for vertex in contour.vertices
            if contour.locate(vertex) is not Location.BOUNDARY] == []
