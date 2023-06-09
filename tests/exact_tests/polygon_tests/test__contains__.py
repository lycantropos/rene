from hypothesis import given

from rene import Location
from rene.exact import (Point,
                        Polygon)
from tests.utils import equivalence
from . import strategies


@given(strategies.polygons, strategies.points)
def test_basic(polygon: Polygon, point: Point) -> None:
    result = point in polygon

    assert isinstance(result, bool)


@given(strategies.polygons, strategies.points)
def test_alternatives(polygon: Polygon, point: Point) -> None:
    result = point in polygon

    assert equivalence(result, polygon.locate(point) is not Location.EXTERIOR)
