from typing import Sequence

import pytest
from hypothesis import given

from rene.exact import Multipolygon, Polygon
from tests.utils import are_sequences_equivalent

from . import strategies


@given(strategies.multipolygons_polygons)
def test_basic(polygons: Sequence[Polygon]) -> None:
    result = Multipolygon(polygons)

    assert isinstance(result, Multipolygon)
    assert are_sequences_equivalent(result.polygons, polygons)


@given(strategies.invalid_count_multipolygons_polygons)
def test_invalid_polygons_count(polygons: Sequence[Polygon]) -> None:
    with pytest.raises(ValueError):
        Multipolygon(polygons)
