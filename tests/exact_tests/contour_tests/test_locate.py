from hypothesis import given

from rene import Location
from rene.exact import Contour, Point
from tests.utils import (
    reverse_contour_coordinates,
    reverse_contour_vertices,
    reverse_point_coordinates,
)

from . import strategies


@given(strategies.contours, strategies.points)
def test_basic(contour: Contour, point: Point) -> None:
    result = contour.locate(point)

    assert isinstance(result, Location)


@given(strategies.contours)
def test_vertices(contour: Contour) -> None:
    assert [
        vertex
        for vertex in contour.vertices
        if contour.locate(vertex) is not Location.BOUNDARY
    ] == []


@given(strategies.contours, strategies.points)
def test_reversals(contour: Contour, point: Point) -> None:
    assert contour.locate(point) is reverse_contour_vertices(contour).locate(
        point
    )
    assert contour.locate(point) is reverse_contour_coordinates(
        contour
    ).locate(reverse_point_coordinates(point))
