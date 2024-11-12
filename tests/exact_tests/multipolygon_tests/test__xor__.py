from hypothesis import given

from rene.exact import Empty, Multipolygon, Polygon
from tests.exact_tests.hints import MaybeShapedCompound
from tests.utils import reverse_multipolygon_polygons

from . import strategies


@given(strategies.multipolygons, strategies.maybe_shaped_compounds)
def test_basic(first: Multipolygon, second: MaybeShapedCompound) -> None:
    result = first ^ second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.multipolygons, strategies.maybe_shaped_compounds)
def test_reversals(first: Multipolygon, second: MaybeShapedCompound) -> None:
    result = first ^ second

    assert result == reverse_multipolygon_polygons(first) ^ second
