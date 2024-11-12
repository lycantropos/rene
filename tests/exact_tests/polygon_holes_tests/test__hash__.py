from collections.abc import Sequence

from hypothesis import given

from rene.exact import Contour
from tests.utils import implication

from . import strategies


@given(strategies.polygons_holes)
def test_determinism(holes: Sequence[Contour]) -> None:
    result = hash(holes)

    assert result == hash(holes)


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_preserving_equality(
    first: Sequence[Contour], second: Sequence[Contour]
) -> None:
    assert implication(first == second, hash(first) == hash(second))
