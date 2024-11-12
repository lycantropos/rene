import typing as t

from hypothesis import given

from rene.exact import Polygon
from tests.utils import equivalence

from . import strategies


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_basic(polygons: t.Sequence[Polygon], polygon: Polygon) -> None:
    result = polygon in polygons

    assert isinstance(result, bool)


@given(strategies.multipolygons_polygons, strategies.polygons)
def test_alternatives(polygons: t.Sequence[Polygon], polygon: Polygon) -> None:
    result = polygon in polygons

    assert equivalence(result, polygon in iter(polygons))
    assert equivalence(result, polygon in list(polygons))
    assert equivalence(result, polygon in tuple(polygons))
