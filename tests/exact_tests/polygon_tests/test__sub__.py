from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.utils import (Compound,
                         equivalence,
                         reverse_compound_coordinates,
                         reverse_polygon_coordinates,
                         reverse_polygon_holes)
from . import strategies


@given(strategies.polygons, strategies.compounds)
def test_basic(first: Polygon, second: Compound) -> None:
    result = first - second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons, strategies.compounds)
def test_commutative_case(first: Polygon, second: Compound) -> None:
    result = first - second

    assert equivalence(result == second - first, first == second)


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_difference_subtrahend(first: Polygon,
                               second: Compound,
                               third: Compound) -> None:
    assert first - (second - third) == (first - second) | (first & third)


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_intersection_minuend(first: Polygon,
                              second: Compound,
                              third: Compound) -> None:
    assert (first & second) - third == first & (second - third)


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_intersection_subtrahend(first: Polygon,
                                 second: Compound,
                                 third: Compound) -> None:
    assert first - (second & third) == (first - second) | (first - third)


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_union_subtrahend(first: Polygon,
                          second: Compound,
                          third: Compound) -> None:
    assert first - (second | third) == (first - second) & (first - third)


@given(strategies.polygons, strategies.compounds)
def test_reversals(first: Polygon, second: Compound) -> None:
    result = first - second

    assert result == reverse_polygon_holes(first) - second
    assert result == reverse_compound_coordinates(
            reverse_polygon_coordinates(first)
            - reverse_compound_coordinates(second)
    )
