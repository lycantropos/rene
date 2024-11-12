from collections.abc import Sequence

import pytest
from hypothesis import given

from rene.exact import Polygon

from . import strategies


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_basic_default_range(
    polygons: Sequence[Polygon], polygon: Polygon
) -> None:
    try:
        result = polygons.index(polygon)
    except ValueError:
        assert polygon not in polygons
    else:
        assert isinstance(result, int)
        assert result in range(len(polygons))
        assert polygons[result] == polygon


@given(
    strategies.multipolygons_polygons,
    strategies.polygons,
    strategies.indices,
    strategies.indices,
)
def test_basic_custom_range(
    polygons: Sequence[Polygon], polygon: Polygon, start: int, stop: int
) -> None:
    try:
        result = polygons.index(polygon, start, stop)
    except ValueError:
        assert polygon not in polygons[start:stop]
    else:
        assert isinstance(result, int)
        assert result in range(start, stop)
        assert polygons[result] == polygon


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_alternatives_default_range(
    polygons: Sequence[Polygon], polygon: Polygon
) -> None:
    try:
        result = polygons.index(polygon)
    except ValueError:
        with pytest.raises(ValueError):
            list(polygons).index(polygon)
        with pytest.raises(ValueError):
            tuple(polygons).index(polygon)
    else:
        assert result == list(polygons).index(polygon)
        assert result == tuple(polygons).index(polygon)


@given(
    strategies.multipolygons_polygons,
    strategies.polygons,
    strategies.indices,
    strategies.indices,
)
def test_alternatives_custom_range(
    polygons: Sequence[Polygon], polygon: Polygon, start: int, stop: int
) -> None:
    try:
        result = polygons.index(polygon, start, stop)
    except ValueError:
        with pytest.raises(ValueError):
            list(polygons).index(polygon, start, stop)
    else:
        assert result == list(polygons).index(polygon, start, stop)
        assert result == tuple(polygons).index(polygon, start, stop)
