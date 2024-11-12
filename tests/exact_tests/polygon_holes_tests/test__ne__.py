from collections.abc import Sequence

from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence

from . import strategies


@given(strategies.polygons_holes)
def test_irreflexivity(holes: Sequence[Contour]) -> None:
    assert holes == holes


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_symmetry(first: Sequence[Contour], second: Sequence[Contour]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_equivalents(
    first: Sequence[Contour], second: Sequence[Contour]
) -> None:
    assert equivalence(first != second, first != second)
