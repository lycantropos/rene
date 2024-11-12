from collections import abc

from hypothesis import given

from rene.exact import Contour, Polygon

from . import strategies


@given(strategies.polygons)
def test_basic(polygon: Polygon) -> None:
    result = polygon.holes

    assert isinstance(result, abc.Sequence)
    assert all(isinstance(element, Contour) for element in result)
