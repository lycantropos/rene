from hypothesis import given

from tests.exact_tests.hints import Compound
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.compounds)
def test_idempotence(compound: Compound) -> None:
    result = compound & compound

    assert result == compound


@given(strategies.compounds, strategies.compounds)
def test_absorption_identity(first: Compound, second: Compound) -> None:
    assert (first | second) & first == first


@given(strategies.compounds, strategies.compounds)
def test_commutativity(first: Compound, second: Compound) -> None:
    result = first & second

    assert result == second & first


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_associativity(first: Compound,
                       second: Compound,
                       third: Compound) -> None:
    assert (first & second) & third == first & (second & third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_difference_operand(first: Compound,
                            second: Compound,
                            third: Compound) -> None:
    assert (first - second) & third == (first & third) - second


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_distribution_over_union(first: Compound,
                                 second: Compound,
                                 third: Compound) -> None:
    assert first & (second | third) == (first & second) | (first & third)


@given(strategies.compounds, strategies.compounds)
def test_alternatives(first: Compound, second: Compound) -> None:
    result = first & second

    assert result == first - (first - second)


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    result = first & second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            & reverse_compound_coordinates(second)
    )
