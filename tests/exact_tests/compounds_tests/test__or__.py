from hypothesis import given

from tests.exact_tests.hints import (ClosedCompoundsPair,
                                     ClosedCompoundsTriplet,
                                     MaybeShapedCompound)
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.maybe_shaped_compounds)
def test_idempotence(compound: MaybeShapedCompound) -> None:
    assert compound | compound == compound


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds)
def test_absorption_identity(first: MaybeShapedCompound,
                             second: MaybeShapedCompound) -> None:
    assert first | (first & second) == first


@given(strategies.closed_compounds_pairs)
def test_commutativity(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    assert first | second == second | first


@given(strategies.closed_compounds_triplets)
def test_associativity(triplet: ClosedCompoundsTriplet) -> None:
    first, second, third = triplet

    assert (first | second) | third == first | second | third


@given(strategies.closed_compounds_triplets)
def test_difference_operand(triplet: ClosedCompoundsTriplet) -> None:
    first, second, third = triplet

    assert (first - second) | third == (first | third) - (second - third)


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds,
       strategies.maybe_shaped_compounds)
def test_distribution_over_intersection(first: MaybeShapedCompound,
                                        second: MaybeShapedCompound,
                                        third: MaybeShapedCompound) -> None:
    assert first | (second & third) == (first | second) & (first | third)


@given(strategies.closed_compounds_pairs)
def test_equivalents(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    result = first | second

    assert result == (first ^ second) ^ (first & second)


@given(strategies.closed_compounds_pairs)
def test_reversals(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    result = first | second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            | reverse_compound_coordinates(second)
    )
