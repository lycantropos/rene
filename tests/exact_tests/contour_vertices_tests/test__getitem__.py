from collections import abc
from collections.abc import Sequence

from hypothesis import given

from rene.exact import Point

from . import strategies


@given(strategies.contours_vertices, strategies.indices)
def test_basic_index(vertices: Sequence[Point], item: int) -> None:
    try:
        result = vertices[item]
    except IndexError:
        assert len(vertices) <= abs(item)
    else:
        assert isinstance(result, Point)


@given(strategies.contours_vertices, strategies.slices)
def test_basic_slice(vertices: Sequence[Point], item: slice) -> None:
    result = vertices[item]

    assert isinstance(result, abc.Sequence)


@given(strategies.contours_vertices, strategies.slices)
def test_slice_commutativity_with_list(
    vertices: Sequence[Point], item: slice
) -> None:
    assert list(vertices[item]) == list(vertices)[item]


@given(strategies.contours_vertices)
def test_shallow_copy(vertices: Sequence[Point]) -> None:
    result = vertices[::]

    assert result is not vertices
    assert result == vertices


@given(strategies.contours_vertices)
def test_reversed(vertices: Sequence[Point]) -> None:
    result = vertices[::-1]

    assert result != vertices
    assert len(result) == len(vertices)
    assert [
        point
        for index, (point, reversed_point) in (
            enumerate(zip(result, reversed(vertices)))
        )
        if point != reversed_point
    ] == []


@given(strategies.contours_vertices)
def test_reversed_idempotence(vertices: Sequence[Point]) -> None:
    result = vertices[::-1]

    assert result[::-1] == vertices


@given(strategies.contours_vertices, strategies.slices, strategies.slices)
def test_consecutive_slicing(
    vertices: Sequence[Point], item: slice, next_item: slice
) -> None:
    result = vertices[item]
    next_result = result[next_item]

    assert len(result) >= len(next_result)
    assert all(element in result for element in next_result)
