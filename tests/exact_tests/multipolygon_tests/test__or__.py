from hypothesis import given

from rene.exact import (Multipolygon,
                        Polygon)
from tests.utils import (reverse_compound_coordinates,
                         reverse_multipolygon,
                         reverse_multipolygon_coordinates)
from . import strategies


@given(strategies.multipolygons, strategies.multipolygons)
def test_basic(first: Multipolygon, second: Multipolygon) -> None:
    result = first | second

    assert isinstance(result, (Multipolygon, Polygon))


@given(strategies.multipolygons)
def test_idempotence(multipolygon: Multipolygon) -> None:
    assert multipolygon | multipolygon == multipolygon


@given(strategies.multipolygons, strategies.multipolygons)
def test_absorption_identity(first: Multipolygon,
                             second: Multipolygon) -> None:
    assert first | (first & second) == first


@given(strategies.multipolygons, strategies.multipolygons)
def test_commutativity(first: Multipolygon, second: Multipolygon) -> None:
    assert first | second == second | first


@given(strategies.multipolygons, strategies.multipolygons,
       strategies.multipolygons)
def test_associativity(first: Multipolygon,
                       second: Multipolygon,
                       third: Multipolygon) -> None:
    assert (first | second) | third == first | second | third


@given(strategies.multipolygons, strategies.multipolygons,
       strategies.multipolygons)
def test_difference_operand(first: Multipolygon,
                            second: Multipolygon,
                            third: Multipolygon) -> None:
    assert (first - second) | third == (first | third) - (second - third)


@given(strategies.multipolygons, strategies.multipolygons,
       strategies.multipolygons)
def test_distribution_over_intersection(first: Multipolygon,
                                        second: Multipolygon,
                                        third: Multipolygon) -> None:
    assert first | (second & third) == (first | second) & (first | third)


@given(strategies.multipolygons, strategies.multipolygons)
def test_equivalents(first: Multipolygon, second: Multipolygon) -> None:
    assert first | second == (first ^ second) ^ (first & second)


@given(strategies.multipolygons, strategies.multipolygons)
def test_reversals(first: Multipolygon, second: Multipolygon) -> None:
    result = first | second

    assert result == reverse_multipolygon(first) | second
    assert result == first | reverse_multipolygon(second)
    assert result == reverse_compound_coordinates(
            reverse_multipolygon_coordinates(first)
            | reverse_multipolygon_coordinates(second)
    )
