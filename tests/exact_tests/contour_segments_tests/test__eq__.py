import typing as t

from hypothesis import given

from rene.exact import Segment
from tests.utils import (equivalence,
                         implication,
                         reverse_sequence)
from . import strategies


@given(strategies.contours_segments)
def test_reflexivity(segments: t.Sequence[Segment]) -> None:
    assert segments == segments


@given(strategies.contours_segments, strategies.contours_segments)
def test_symmetry(first: t.Sequence[Segment],
                  second: t.Sequence[Segment]) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.contours_segments, strategies.contours_segments,
       strategies.contours_segments)
def test_transitivity(first: t.Sequence[Segment],
                      second: t.Sequence[Segment],
                      third: t.Sequence[Segment]) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.contours_segments, strategies.contours_segments)
def test_alternatives(first: t.Sequence[Segment],
                      second: t.Sequence[Segment]) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.contours_segments, strategies.contours_segments)
def test_reversals(first: t.Sequence[Segment],
                   second: t.Sequence[Segment]) -> None:
    assert equivalence(first == second,
                       reverse_sequence(first) == reverse_sequence(second))
