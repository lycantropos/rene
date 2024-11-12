import typing as t

from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence

from . import strategies


@given(strategies.contours_segments, strategies.segments)
def test_basic(segments: t.Sequence[Segment], segment: Segment) -> None:
    result = segments.count(segment)

    assert isinstance(result, int)
    assert result in range(len(segments))
    assert equivalence(result == 0, segment not in segments)


@given(strategies.contours_segments, strategies.segments)
def test_alternatives(segments: t.Sequence[Segment], segment: Segment) -> None:
    result = segments.count(segment)

    assert result == list(segments).count(segment)
    assert result == tuple(segments).count(segment)
