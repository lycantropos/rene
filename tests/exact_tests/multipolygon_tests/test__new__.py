from typing import Sequence

import pytest
from hypothesis import given

from rene.exact import (Multipolygon,
                        Point)
from . import strategies


@given(strategies.multipolygons_polygons)
def test_basic(polygons: Sequence[Point]) -> None:
    result = Multipolygon(polygons)

    assert isinstance(result, Multipolygon)
    assert result.polygons == polygons


@given(strategies.invalid_count_multipolygons_polygons)
def test_invalid_polygons_count(polygons: Sequence[Point]) -> None:
    with pytest.raises(ValueError):
        Multipolygon(polygons)
