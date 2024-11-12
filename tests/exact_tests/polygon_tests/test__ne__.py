from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence

from . import strategies


@given(strategies.polygons)
def test_irreflexivity(polygon: Polygon) -> None:
    assert polygon == polygon


@given(strategies.polygons, strategies.polygons)
def test_symmetry(first: Polygon, second: Polygon) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.polygons, strategies.polygons)
def test_equivalents(first: Polygon, second: Polygon) -> None:
    assert equivalence(first != second, first != second)
