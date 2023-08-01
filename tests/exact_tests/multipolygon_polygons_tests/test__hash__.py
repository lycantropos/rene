import typing as t

from hypothesis import given

from rene.exact import Polygon
from tests.utils import implication
from . import strategies


@given(strategies.multipolygons_polygons)
def test_determinism(polygons: t.Sequence[Polygon]) -> None:
    result = hash(polygons)

    assert result == hash(polygons)


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_preserving_equality(first: t.Sequence[Polygon],
                             second: t.Sequence[Polygon]) -> None:
    assert implication(first == second, hash(first) == hash(second))
