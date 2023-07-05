from hypothesis import given

from tests.exact_tests.hints import Compound
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.compounds)
def test_idempotence(compound: Compound) -> None:
    assert compound | compound == compound


@given(strategies.compounds, strategies.compounds)
def test_absorption_identity(first: Compound, second: Compound) -> None:
    assert first | (first & second) == first


@given(strategies.compounds, strategies.compounds)
def test_commutativity(first: Compound, second: Compound) -> None:
    assert first | second == second | first


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_associativity(first: Compound,
                       second: Compound,
                       third: Compound) -> None:
    assert (first | second) | third == first | second | third


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_difference_operand(first: Compound,
                            second: Compound,
                            third: Compound) -> None:
    assert (first - second) | third == (first | third) - (second - third)


@given(strategies.compounds, strategies.compounds, strategies.compounds)
def test_distribution_over_intersection(first: Compound,
                                        second: Compound,
                                        third: Compound) -> None:
    assert first | (second & third) == (first | second) & (first | third)


@given(strategies.compounds, strategies.compounds)
def test_equivalents(first: Compound, second: Compound) -> None:
    assert first | second == (first ^ second) ^ (first & second)


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    result = first | second

    assert result == reverse_compound_coordinates(
            reverse_compound_coordinates(first)
            | reverse_compound_coordinates(second)
    )
