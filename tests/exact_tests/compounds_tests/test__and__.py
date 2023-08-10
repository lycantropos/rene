from hypothesis import given

from tests.exact_tests.hints import (ClosedCompoundsTripletT,
                                     ClosedIdempotentCompoundT,
                                     Compound,
                                     MaybeShapedCompound)
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.closed_idempotent_compounds)
def test_idempotence(compound: ClosedIdempotentCompoundT) -> None:
    result = compound & compound

    assert result == compound


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds)
def test_absorption_identity(first: MaybeShapedCompound,
                             second: MaybeShapedCompound) -> None:
    assert (first | second) & first == first


@given(strategies.compounds, strategies.compounds)
def test_commutativity(first: Compound, second: Compound) -> None:
    assert first & second == second & first


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_associativity(first: Compound,
                       second: Compound,
                       third: Compound) -> None:
    assert (first & second) & third == first & (second & third)


@given(strategies.closed_compounds_triplets)
def test_difference_operand(triplet: ClosedCompoundsTripletT) -> None:
    first, second, third = triplet

    assert (first - second) & third == (first & third) - second


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds,
       strategies.maybe_shaped_compounds)
def test_distribution_over_union(first: MaybeShapedCompound,
                                 second: MaybeShapedCompound,
                                 third: MaybeShapedCompound) -> None:
    assert first & (second | third) == (first & second) | (first & third)


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds)
def test_alternatives(first: MaybeShapedCompound,
                      second: MaybeShapedCompound) -> None:
    result = first & second

    assert result == first - (first - second)


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    result = first & second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            & reverse_compound_coordinates(second)
    )
