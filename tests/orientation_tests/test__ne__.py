from hypothesis import given

from rene import Orientation
from tests.utils import equivalence
from . import strategies


@given(strategies.orientations)
def test_irreflexivity(orientation: Orientation) -> None:
    assert not orientation != orientation


@given(strategies.orientations, strategies.orientations)
def test_symmetry(first: Orientation, second: Orientation) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.orientations, strategies.orientations)
def test_equivalents(first: Orientation, second: Orientation) -> None:
    assert equivalence(first != second, not first == second)
