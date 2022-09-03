from functools import reduce
from operator import or_, xor

from gon import base
from hypothesis import given

from rene.exact import (ConstrainedDelaunayTriangulation,
                        Polygon)
from . import strategies
from ... import to_polygon


@given(strategies.polygons)
def test_basic(polygon: Polygon) -> None:
    result = ConstrainedDelaunayTriangulation.from_polygon(polygon)

    assert isinstance(result, ConstrainedDelaunayTriangulation)


@given(strategies.polygons)
def test_border(polygon: Polygon) -> None:
    result = ConstrainedDelaunayTriangulation.from_polygon(polygon)

    assert result.border == polygon.border


@given(strategies.polygons)
def test_triangles(polygon: Polygon) -> None:
    triangulation = ConstrainedDelaunayTriangulation.from_polygon(polygon)

    assert triangulation.border == polygon.border
    triangles = triangulation.triangles
    # draw(polygon, triangulation)
    assert triangles
    united = to_polygon(reduce(xor, [base.Polygon(border, [])
                                     for border in triangles]))
    assert united == polygon
