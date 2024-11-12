from hypothesis import given

from rene.exact import Empty, Multipolygon, Multisegment, Polygon, Segment
from tests.exact_tests.hints import (
    Compound,
    MaybeLinearCompound,
    MaybeShapedCompound,
)
from tests.utils import reverse_multipolygon_polygons

from . import strategies


@given(strategies.multipolygons, strategies.maybe_shaped_compounds)
def test_basic_maybe_shaped_compound_operand(
    first: Multipolygon, second: MaybeShapedCompound
) -> None:
    result = first & second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.multipolygons, strategies.maybe_linear_compounds)
def test_basic_maybe_linear_compound_operand(
    first: Multipolygon, second: MaybeLinearCompound
) -> None:
    result = first & second

    assert isinstance(result, (Empty, Multisegment, Segment))


@given(strategies.multipolygons, strategies.compounds)
def test_reversals(first: Multipolygon, second: Compound) -> None:
    result = first & second

    assert result == reverse_multipolygon_polygons(first) & second
