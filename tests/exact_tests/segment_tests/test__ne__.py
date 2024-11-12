from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence

from . import strategies


@given(strategies.segments)
def test_irreflexivity(segment: Segment) -> None:
    assert segment == segment


@given(strategies.segments, strategies.segments)
def test_symmetry(first: Segment, second: Segment) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.segments, strategies.segments)
def test_equivalents(first: Segment, second: Segment) -> None:
    assert equivalence(first != second, first != second)
