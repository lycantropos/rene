from hypothesis import given

from rene.exact import Polygon
from . import strategies


@given(strategies.polygons)
def test_basic(polygon: Polygon) -> None:
    result = polygon.holes_count

    assert isinstance(result, int)


@given(strategies.polygons)
def test_value(polygon: Polygon) -> None:
    result = polygon.holes_count

    assert result == len(polygon.holes)
