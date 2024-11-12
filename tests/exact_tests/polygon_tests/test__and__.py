from hypothesis import given

from rene.exact import Empty, Multipolygon, Multisegment, Polygon, Segment
from tests.exact_tests.hints import (
    Compound,
    MaybeLinearCompound,
    MaybeShapedCompound,
)
from tests.utils import reverse_polygon_holes

from . import strategies


@given(strategies.polygons, strategies.maybe_shaped_compounds)
def test_basic_maybe_shaped_compound_operand(
    first: Polygon, second: MaybeShapedCompound
) -> None:
    result = first & second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons, strategies.maybe_linear_compounds)
def test_basic_maybe_linear_compound_operand(
    first: Polygon, second: MaybeLinearCompound
) -> None:
    result = first & second

    assert isinstance(result, (Empty, Multisegment, Segment))


@given(strategies.polygons, strategies.compounds)
def test_reversals(first: Polygon, second: Compound) -> None:
    result = first & second

    assert result == reverse_polygon_holes(first) & second
