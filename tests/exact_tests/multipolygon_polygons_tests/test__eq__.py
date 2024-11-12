import typing as t

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence, implication, reverse_sequence

from . import strategies


@given(strategies.multipolygons_polygons)
def test_reflexivity(polygons: t.Sequence[Polygon]) -> None:
    assert polygons == polygons


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_symmetry(
    first: t.Sequence[Polygon], second: t.Sequence[Polygon]
) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.multipolygons_polygons,
    strategies.multipolygons_polygons,
    strategies.multipolygons_polygons,
)
def test_transitivity(
    first: t.Sequence[Polygon],
    second: t.Sequence[Polygon],
    third: t.Sequence[Polygon],
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_alternatives(
    first: t.Sequence[Polygon], second: t.Sequence[Polygon]
) -> None:
    assert equivalence(first == second, first == second)


@given(strategies.multipolygons_polygons, strategies.multipolygons_polygons)
def test_reversals(
    first: t.Sequence[Polygon], second: t.Sequence[Polygon]
) -> None:
    assert equivalence(
        first == second, reverse_sequence(first) == reverse_sequence(second)
    )
