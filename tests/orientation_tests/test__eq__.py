from hypothesis import given

from rene.enums import Orientation
from tests.utils import equivalence, implication

from . import strategies


@given(strategies.orientations)
def test_reflexivity(orientation: Orientation) -> None:
    assert orientation == orientation


@given(strategies.orientations, strategies.orientations)
def test_symmetry(first: Orientation, second: Orientation) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.orientations, strategies.orientations, strategies.orientations
)
def test_transitivity(
    first: Orientation, second: Orientation, third: Orientation
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.orientations, strategies.orientations)
def test_alternatives(first: Orientation, second: Orientation) -> None:
    assert equivalence(first == second, first == second)
