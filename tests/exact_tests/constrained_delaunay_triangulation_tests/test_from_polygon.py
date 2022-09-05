from hypothesis import given

from rene.exact import (ConstrainedDelaunayTriangulation,
                        Polygon)
from . import strategies


@given(strategies.polygons)
def test_basic(polygon: Polygon) -> None:
    result = ConstrainedDelaunayTriangulation.from_polygon(polygon)

    assert isinstance(result, ConstrainedDelaunayTriangulation)


@given(strategies.polygons)
def test_border(polygon: Polygon) -> None:
    result = ConstrainedDelaunayTriangulation.from_polygon(polygon)

    assert result.border == polygon.border
