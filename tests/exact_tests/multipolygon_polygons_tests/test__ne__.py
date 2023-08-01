import typing as t

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence
from . import strategies


@given(strategies.multipolygons_polygons)
def test_irreflexivity(multipolygon: t.Sequence[Polygon]) -> None:
    assert not multipolygon != multipolygon


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_symmetry(first: t.Sequence[Polygon],
                  second: t.Sequence[Polygon]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_equivalents(first: t.Sequence[Polygon],
                     second: t.Sequence[Polygon]) -> None:
    assert equivalence(first != second, not first == second)
