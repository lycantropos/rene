from hypothesis import given

from rene.exact import Segment
from tests.utils import (equivalence,
                         implication,
                         reverse_segment_endpoints)
from . import strategies


@given(strategies.segments)
def test_reflexivity(segment: Segment) -> None:
    assert segment == segment


@given(strategies.segments, strategies.segments)
def test_symmetry(first: Segment, second: Segment) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.segments, strategies.segments, strategies.segments)
def test_transitivity(first: Segment, second: Segment, third: Segment) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.segments, strategies.segments)
def test_alternatives(first: Segment, second: Segment) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.segments)
def test_endpoints_reversal(segment: Segment) -> None:
    assert segment == reverse_segment_endpoints(segment)
