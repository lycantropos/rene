import typing as t

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence
from . import strategies


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_basic(polygons: t.Sequence[Polygon], polygon: Polygon) -> None:
    result = polygons.count(polygon)

    assert isinstance(result, int)
    assert result in range(len(polygons))
    assert equivalence(result == 0, polygon not in polygons)


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_alternatives(polygons: t.Sequence[Polygon], polygon: Polygon) -> None:
    result = polygons.count(polygon)

    assert result == list(polygons).count(polygon)
    assert result == tuple(polygons).count(polygon)
