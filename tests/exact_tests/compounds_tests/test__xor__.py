from hypothesis import given

from rene.exact import Empty
from tests.exact_tests.hints import (ClosedCompoundsPair,
                                     ClosedCompoundsTriplet,
                                     CompoundT,
                                     MaybeShapedCompound)
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.compounds)
def test_self_inverse(compound: CompoundT) -> None:
    result = compound ^ compound

    assert isinstance(result, Empty)


@given(strategies.closed_compounds_pairs)
def test_commutativity(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    result = first ^ second

    assert result == second ^ first


@given(strategies.closed_compounds_triplets)
def test_associativity(triplet: ClosedCompoundsTriplet) -> None:
    first, second, third = triplet

    assert (first ^ second) ^ third == first ^ (second ^ third)


@given(strategies.maybe_shaped_compounds, strategies.maybe_shaped_compounds,
       strategies.maybe_shaped_compounds)
def test_repeated(first: MaybeShapedCompound,
                  second: MaybeShapedCompound,
                  third: MaybeShapedCompound) -> None:
    assert (first ^ second) ^ (second ^ third) == first ^ third


@given(strategies.closed_compounds_pairs)
def test_alternatives(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    result = first ^ second

    assert result == (first - second) | (second - first)
    assert result == (first | second) - (second & first)


@given(strategies.closed_compounds_pairs)
def test_reversals(pair: ClosedCompoundsPair) -> None:
    first, second = pair

    result = first ^ second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            ^ reverse_compound_coordinates(second)
    )
