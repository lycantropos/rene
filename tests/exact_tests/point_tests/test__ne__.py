from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence

from . import strategies


@given(strategies.points)
def test_irreflexivity(point: Point) -> None:
    assert point == point


@given(strategies.points, strategies.points)
def test_symmetry(first: Point, second: Point) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.points, strategies.points)
def test_equivalents(first: Point, second: Point) -> None:
    assert equivalence(first != second, first != second)
