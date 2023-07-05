from hypothesis import given

from rene.exact import Empty
from tests.exact_tests.hints import Compound
from tests.utils import (equivalence,
                         reverse_compound_coordinates)
from . import strategies


@given(strategies.compounds)
def test_self_inverse(compound: Compound) -> None:
    result = compound - compound

    assert isinstance(result, Empty)


@given(strategies.compounds, strategies.compounds)
def test_commutative_case(first: Compound, second: Compound) -> None:
    result = first - second

    assert equivalence(result == second - first, first == second)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_difference_subtrahend(first: Compound,
                               second: Compound,
                               third: Compound) -> None:
    assert first - (second - third) == (first - second) | (first & third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_intersection_minuend(first: Compound,
                              second: Compound,
                              third: Compound) -> None:
    assert (first & second) - third == first & (second - third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_intersection_subtrahend(first: Compound,
                                 second: Compound,
                                 third: Compound) -> None:
    assert first - (second & third) == (first - second) | (first - third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_union_subtrahend(first: Compound,
                          second: Compound,
                          third: Compound) -> None:
    assert first - (second | third) == (first - second) & (first - third)


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    result = first - second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            - reverse_compound_coordinates(second)
    )
