from collections import abc
from collections.abc import Sequence

from hypothesis import given

from rene.exact import Segment

from . import strategies


@given(strategies.multisegments_segments, strategies.indices)
def test_basic_index(segments: Sequence[Segment], item: int) -> None:
    try:
        result = segments[item]
    except IndexError:
        assert len(segments) <= abs(item)
    else:
        assert isinstance(result, Segment)


@given(strategies.multisegments_segments, strategies.slices)
def test_basic_slice(segments: Sequence[Segment], item: slice) -> None:
    result = segments[item]

    assert isinstance(result, abc.Sequence)


@given(strategies.multisegments_segments, strategies.slices)
def test_slice_commutativity_with_list(
    segments: Sequence[Segment], item: slice
) -> None:
    assert list(segments[item]) == list(segments)[item]


@given(strategies.multisegments_segments)
def test_shallow_copy(segments: Sequence[Segment]) -> None:
    result = segments[::]

    assert result is not segments
    assert result == segments


@given(strategies.multisegments_segments)
def test_reversed(segments: Sequence[Segment]) -> None:
    result = segments[::-1]

    assert result != segments
    assert len(result) == len(segments)
    assert [
        segment
        for index, (segment, reversed_segment) in (
            enumerate(zip(result, reversed(segments), strict=False))
        )
        if segment != reversed_segment
    ] == []


@given(strategies.multisegments_segments)
def test_reversed_idempotence(segments: Sequence[Segment]) -> None:
    result = segments[::-1]

    assert result[::-1] == segments


@given(strategies.multisegments_segments, strategies.slices, strategies.slices)
def test_consecutive_slicing(
    segments: Sequence[Segment], item: slice, next_item: slice
) -> None:
    result = segments[item]
    next_result = result[next_item]

    assert len(result) >= len(next_result)
    assert all(element in result for element in next_result)
