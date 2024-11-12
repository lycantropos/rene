import typing as t

import pytest
from hypothesis import given

from rene.exact import Segment

from . import strategies


@given(strategies.contours_segments, strategies.segments)
def test_basic_default_range(
    segments: t.Sequence[Segment], segment: Segment
) -> None:
    try:
        result = segments.index(segment)
    except ValueError:
        assert segment not in segments
    else:
        assert isinstance(result, int)
        assert result in range(len(segments))
        assert segments[result] == segment


@given(
    strategies.contours_segments,
    strategies.segments,
    strategies.indices,
    strategies.indices,
)
def test_basic_custom_range(
    segments: t.Sequence[Segment], segment: Segment, start: int, stop: int
) -> None:
    try:
        result = segments.index(segment, start, stop)
    except ValueError:
        assert segment not in segments[start:stop]
    else:
        assert isinstance(result, int)
        assert result in range(start, stop)
        assert segments[result] == segment


@given(strategies.contours_segments, strategies.segments)
def test_alternatives_default_range(
    segments: t.Sequence[Segment], segment: Segment
) -> None:
    try:
        result = segments.index(segment)
    except ValueError:
        with pytest.raises(ValueError):
            list(segments).index(segment)
        with pytest.raises(ValueError):
            tuple(segments).index(segment)
    else:
        assert result == list(segments).index(segment)
        assert result == tuple(segments).index(segment)


@given(
    strategies.contours_segments,
    strategies.segments,
    strategies.indices,
    strategies.indices,
)
def test_alternatives_custom_range(
    segments: t.Sequence[Segment], segment: Segment, start: int, stop: int
) -> None:
    try:
        result = segments.index(segment, start, stop)
    except ValueError:
        with pytest.raises(ValueError):
            list(segments).index(segment, start, stop)
    else:
        assert result == list(segments).index(segment, start, stop)
        assert result == tuple(segments).index(segment, start, stop)
