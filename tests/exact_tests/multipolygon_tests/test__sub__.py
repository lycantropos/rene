from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.utils import (Compound,
                         equivalence,
                         reverse_compound_coordinates,
                         reverse_multipolygon,
                         reverse_multipolygon_coordinates)
from . import strategies


@given(strategies.multipolygons, strategies.compounds)
def test_basic(first: Multipolygon, second: Compound) -> None:
    result = first - second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.multipolygons, strategies.compounds)
def test_commutative_case(first: Multipolygon, second: Compound) -> None:
    result = first - second

    assert equivalence(result == second - first, first == second)


@given(strategies.multipolygons, strategies.compounds, strategies.compounds)
def test_difference_subtrahend(first: Multipolygon,
                               second: Compound,
                               third: Compound) -> None:
    assert first - (second - third) == (first - second) | (first & third)


@given(strategies.multipolygons, strategies.compounds, strategies.compounds)
def test_intersection_minuend(first: Multipolygon,
                              second: Compound,
                              third: Compound) -> None:
    assert (first & second) - third == first & (second - third)


@given(strategies.multipolygons, strategies.compounds, strategies.compounds)
def test_intersection_subtrahend(first: Multipolygon,
                                 second: Compound,
                                 third: Compound) -> None:
    assert first - (second & third) == (first - second) | (first - third)


@given(strategies.multipolygons, strategies.compounds, strategies.compounds)
def test_union_subtrahend(first: Multipolygon,
                          second: Compound,
                          third: Compound) -> None:
    assert first - (second | third) == (first - second) & (first - third)


@given(strategies.multipolygons, strategies.compounds)
def test_reversals(first: Multipolygon, second: Compound) -> None:
    result = first - second

    assert result == reverse_multipolygon(first) - second
    assert result == reverse_compound_coordinates(
            reverse_multipolygon_coordinates(first)
            - reverse_compound_coordinates(second)
    )
