import typing as t

from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence
from . import strategies


@given(strategies.contours_segments, strategies.segments)
def test_basic(segments: t.Sequence[Segment], segment: Segment) -> None:
    result = segment in segments

    assert isinstance(result, bool)


@given(strategies.contours_segments, strategies.segments)
def test_alternatives(segments: t.Sequence[Segment], segment: Segment) -> None:
    result = segment in segments

    assert equivalence(result, segment in iter(segments))
    assert equivalence(result, segment in list(segments))
    assert equivalence(result, segment in tuple(segments))
