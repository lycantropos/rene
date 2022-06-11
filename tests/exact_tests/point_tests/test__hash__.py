from hypothesis import given

from rene.exact import Point
from tests.utils import implication
from . import strategies


@given(strategies.points)
def test_determinism(point: Point) -> None:
    result = hash(point)

    assert result == hash(point)


@given(strategies.points, strategies.points)
def test_preserving_equality(first: Point, second: Point) -> None:
    assert implication(first == second, hash(first) == hash(second))
