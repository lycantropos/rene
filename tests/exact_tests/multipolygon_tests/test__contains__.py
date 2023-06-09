from hypothesis import given

from rene import Location
from rene.exact import (Multipolygon,
                        Point)
from tests.utils import equivalence
from . import strategies


@given(strategies.multipolygons, strategies.points)
def test_basic(multipolygon: Multipolygon, point: Point) -> None:
    result = point in multipolygon

    assert isinstance(result, bool)


@given(strategies.multipolygons, strategies.points)
def test_alternatives(multipolygon: Multipolygon, point: Point) -> None:
    result = point in multipolygon

    assert equivalence(result,
                       multipolygon.locate(point) is not Location.EXTERIOR)
