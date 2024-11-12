from hypothesis import given

from rene import Orientation
from tests.utils import implication

from . import strategies


@given(strategies.orientations)
def test_determinism(orientation: Orientation) -> None:
    result = hash(orientation)

    assert result == hash(orientation)


@given(strategies.orientations, strategies.orientations)
def test_preserving_equality(first: Orientation, second: Orientation) -> None:
    assert implication(first == second, hash(first) == hash(second))
