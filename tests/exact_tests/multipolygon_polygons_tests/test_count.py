from collections.abc import Sequence

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence

from . import strategies


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_basic(polygons: Sequence[Polygon], polygon: Polygon) -> None:
    result = polygons.count(polygon)

    assert isinstance(result, int)
    assert result in range(len(polygons))
    assert equivalence(result == 0, polygon not in polygons)


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_alternatives(polygons: Sequence[Polygon], polygon: Polygon) -> None:
    result = polygons.count(polygon)

    assert result == list(polygons).count(polygon)
    assert result == tuple(polygons).count(polygon)
