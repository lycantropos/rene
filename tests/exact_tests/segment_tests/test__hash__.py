from hypothesis import given

from rene.exact import Segment
from tests.utils import implication

from . import strategies


@given(strategies.segments)
def test_determinism(segment: Segment) -> None:
    result = hash(segment)

    assert result == hash(segment)


@given(strategies.segments, strategies.segments)
def test_preserving_equality(first: Segment, second: Segment) -> None:
    assert implication(first == second, hash(first) == hash(second))
