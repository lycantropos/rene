from hypothesis import given

from rene.exact import Empty
from tests.exact_tests.hints import (
    ClosedCompoundsPairT,
    ClosedCompoundsTripletT,
    CompoundT,
    MaybeShapedCompound,
)
from tests.utils import equivalence, reverse_compound_coordinates

from . import strategies


@given(strategies.compounds)
def test_self_inverse(compound: CompoundT) -> None:
    result = compound - compound

    assert isinstance(result, Empty)


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds)
def test_commutative_case(
    first: MaybeShapedCompound, second: MaybeShapedCompound
) -> None:
    result = first - second

    assert equivalence(result == second - first, first == second)


@given(
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
)
def test_difference_subtrahend(
    first: MaybeShapedCompound,
    second: MaybeShapedCompound,
    third: MaybeShapedCompound,
) -> None:
    assert first - (second - third) == (first - second) | (first & third)


@given(strategies.closed_compounds_triplets)
def test_intersection_minuend(triplet: ClosedCompoundsTripletT) -> None:
    first, second, third = triplet

    assert (first & second) - third == first & (second - third)


@given(
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
)
def test_intersection_subtrahend(
    first: MaybeShapedCompound,
    second: MaybeShapedCompound,
    third: MaybeShapedCompound,
) -> None:
    assert first - (second & third) == (first - second) | (first - third)


@given(
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
    strategies.maybe_shaped_compounds,
)
def test_union_subtrahend(
    first: MaybeShapedCompound,
    second: MaybeShapedCompound,
    third: MaybeShapedCompound,
) -> None:
    assert first - (second | third) == (first - second) & (first - third)


@given(strategies.closed_compounds_pairs)
def test_reversals(pair: ClosedCompoundsPairT) -> None:
    first, second = pair

    result = first - second

    assert result == reverse_compound_coordinates(
        reverse_compound_coordinates(first)
        - reverse_compound_coordinates(second)
    )
