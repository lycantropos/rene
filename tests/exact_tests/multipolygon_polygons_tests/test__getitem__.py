from collections import abc
from collections.abc import Sequence

from hypothesis import given

from rene.exact import Polygon

from . import strategies


@given(strategies.multipolygons_polygons, strategies.indices)
def test_basic_index(polygons: Sequence[Polygon], item: int) -> None:
    try:
        result = polygons[item]
    except IndexError:
        assert len(polygons) <= abs(item)
    else:
        assert isinstance(result, Polygon)


@given(strategies.multipolygons_polygons, strategies.slices)
def test_basic_slice(polygons: Sequence[Polygon], item: slice) -> None:
    result = polygons[item]

    assert isinstance(result, abc.Sequence)


@given(strategies.multipolygons_polygons, strategies.slices)
def test_slice_commutativity_with_list(
    polygons: Sequence[Polygon], item: slice
) -> None:
    assert list(polygons[item]) == list(polygons)[item]


@given(strategies.multipolygons_polygons)
def test_shallow_copy(polygons: Sequence[Polygon]) -> None:
    result = polygons[::]

    assert result is not polygons
    assert result == polygons


@given(strategies.multipolygons_polygons)
def test_reversed(polygons: Sequence[Polygon]) -> None:
    result = polygons[::-1]

    assert result != polygons
    assert len(result) == len(polygons)
    assert [
        polygon
        for index, (polygon, reversed_polygon) in (
            enumerate(zip(result, reversed(polygons), strict=False))
        )
        if polygon != reversed_polygon
    ] == []


@given(strategies.multipolygons_polygons)
def test_reversed_idempotence(polygons: Sequence[Polygon]) -> None:
    result = polygons[::-1]

    assert result[::-1] == polygons


@given(strategies.multipolygons_polygons, strategies.slices, strategies.slices)
def test_consecutive_slicing(
    polygons: Sequence[Polygon], item: slice, next_item: slice
) -> None:
    result = polygons[item]
    next_result = result[next_item]

    assert len(result) >= len(next_result)
    assert all(element in result for element in next_result)
