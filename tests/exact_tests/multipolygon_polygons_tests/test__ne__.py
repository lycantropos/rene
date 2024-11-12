from collections.abc import Sequence

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence

from . import strategies


@given(strategies.multipolygons_polygons)
def test_irreflexivity(polygons: Sequence[Polygon]) -> None:
    assert polygons == polygons


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_symmetry(first: Sequence[Polygon], second: Sequence[Polygon]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_equivalents(
    first: Sequence[Polygon], second: Sequence[Polygon]
) -> None:
    assert equivalence(first != second, first != second)
