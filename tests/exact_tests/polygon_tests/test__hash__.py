from hypothesis import given

from rene.exact import Polygon
from tests.utils import implication
from . import strategies


@given(strategies.polygons)
def test_determinism(polygon: Polygon) -> None:
    result = hash(polygon)

    assert result == hash(polygon)


@given(strategies.polygons, strategies.polygons)
def test_preserving_equality(first: Polygon, second: Polygon) -> None:
    assert implication(first == second, hash(first) == hash(second))
