from hypothesis import given

from rene.exact import Box
from tests.utils import implication
from . import strategies


@given(strategies.boxes)
def test_determinism(box: Box) -> None:
    result = hash(box)

    assert result == hash(box)


@given(strategies.boxes, strategies.boxes)
def test_preserving_equality(first: Box, second: Box) -> None:
    assert implication(first == second, hash(first) == hash(second))
