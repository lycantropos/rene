import typing as t

from hypothesis import given

from rene.exact import Segment
from tests.utils import implication
from . import strategies


@given(strategies.contours_segments)
def test_determinism(segments: t.Sequence[Segment]) -> None:
    result = hash(segments)

    assert result == hash(segments)


@given(strategies.contours_segments, strategies.contours_segments)
def test_preserving_equality(first: t.Sequence[Segment],
                             second: t.Sequence[Segment]) -> None:
    assert implication(first == second, hash(first) == hash(second))
