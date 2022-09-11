from hypothesis import given

from rene.exact import Empty
from tests.utils import (Compound,
                         reverse_compound_coordinates)
from . import strategies


@given(strategies.compounds)
def test_self_inverse(compound: Compound) -> None:
    result = compound ^ compound

    assert isinstance(result, Empty)


@given(strategies.compounds, strategies.compounds)
def test_commutativity(first: Compound, second: Compound) -> None:
    result = first ^ second

    assert result == second ^ first


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_associativity(first: Compound,
                       second: Compound,
                       third: Compound) -> None:
    assert (first ^ second) ^ third == first ^ (second ^ third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_repeated(first: Compound,
                  second: Compound,
                  third: Compound) -> None:
    assert (first ^ second) ^ (second ^ third) == first ^ third


@given(strategies.compounds, strategies.compounds)
def test_alternatives(first: Compound, second: Compound) -> None:
    result = first ^ second

    assert result == (first - second) | (second - first)
    assert result == (first | second) - (second & first)


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    result = first ^ second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            ^ reverse_compound_coordinates(second)
    )
