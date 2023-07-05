from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.exact_tests.hints import Compound
from tests.utils import reverse_polygon_holes
from . import strategies


@given(strategies.polygons, strategies.compounds)
def test_basic(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons, strategies.compounds)
def test_reversals(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert result == reverse_polygon_holes(first) ^ second
