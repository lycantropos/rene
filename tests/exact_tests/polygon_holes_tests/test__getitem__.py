import typing as t
from collections import abc

from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence
from . import strategies


@given(strategies.polygons_holes, strategies.indices)
def test_basic_index(holes: t.Sequence[Contour], item: int) -> None:
    try:
        result = holes[item]
    except IndexError:
        assert len(holes) <= abs(item)
    else:
        assert isinstance(result, Contour)


@given(strategies.polygons_holes, strategies.slices)
def test_basic_slice(holes: t.Sequence[Contour], item: slice) -> None:
    result = holes[item]

    assert isinstance(result, abc.Sequence)


@given(strategies.polygons_holes, strategies.slices)
def test_slice_commutativity_with_list(holes: t.Sequence[Contour],
                                       item: slice) -> None:
    assert list(holes[item]) == list(holes)[item]


@given(strategies.polygons_holes)
def test_shallow_copy(holes: t.Sequence[Contour]) -> None:
    result = holes[::]

    assert result is not holes
    assert result == holes


@given(strategies.polygons_holes)
def test_reversed(holes: t.Sequence[Contour]) -> None:
    result = holes[::-1]

    assert equivalence(len(result) == 0, result == holes)
    assert len(result) == len(holes)
    assert [contour
            for index, (contour, reversed_contour) in (
                enumerate(zip(result, reversed(holes)))
            )
            if contour != reversed_contour] == []


@given(strategies.polygons_holes)
def test_reversed_idempotence(holes: t.Sequence[Contour]) -> None:
    result = holes[::-1]

    assert result[::-1] == holes


@given(strategies.polygons_holes, strategies.slices, strategies.slices)
def test_consecutive_slicing(holes: t.Sequence[Contour],
                             item: slice,
                             next_item: slice) -> None:
    result = holes[item]
    next_result = result[next_item]

    assert len(result) >= len(next_result)
    assert all(element in result for element in next_result)
