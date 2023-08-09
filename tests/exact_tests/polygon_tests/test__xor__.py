from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.exact_tests.hints import MaybeShapedCompound
from tests.utils import reverse_polygon_holes
from . import strategies


@given(strategies.polygons, strategies.maybe_shaped_compounds)
def test_basic(first: Polygon, second: MaybeShapedCompound) -> None:
    result = first ^ second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons, strategies.maybe_shaped_compounds)
def test_reversals(first: Polygon, second: MaybeShapedCompound) -> None:
    result = first ^ second

    assert result == reverse_polygon_holes(first) ^ second
