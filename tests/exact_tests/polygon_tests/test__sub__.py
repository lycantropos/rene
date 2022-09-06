from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.utils import (equivalence,
                         reverse_compound_coordinates,
                         reverse_polygon_coordinates,
                         reverse_polygon_holes)
from . import strategies


@given(strategies.polygons, strategies.polygons)
def test_basic(first: Polygon, second: Polygon) -> None:
    result = first - second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons, strategies.polygons)
def test_commutative_case(first: Polygon, second: Polygon) -> None:
    result = first - second

    assert equivalence(result == second - first, first == second)


@given(strategies.polygons, strategies.polygons, strategies.polygons)
def test_difference_subtrahend(first: Polygon,
                               second: Polygon,
                               third: Polygon) -> None:
    assert first - (second - third) == (first - second) | (first & third)


@given(strategies.polygons, strategies.polygons, strategies.polygons)
def test_intersection_minuend(first: Polygon,
                              second: Polygon,
                              third: Polygon) -> None:
    assert (first & second) - third == first & (second - third)


@given(strategies.polygons, strategies.polygons, strategies.polygons)
def test_intersection_subtrahend(first: Polygon,
                                 second: Polygon,
                                 third: Polygon) -> None:
    assert first - (second & third) == (first - second) | (first - third)


@given(strategies.polygons, strategies.polygons, strategies.polygons)
def test_union_subtrahend(first: Polygon,
                          second: Polygon,
                          third: Polygon) -> None:
    assert first - (second | third) == (first - second) & (first - third)


@given(strategies.polygons, strategies.polygons)
def test_reversals(first: Polygon, second: Polygon) -> None:
    result = first - second

    assert result == reverse_polygon_holes(first) - second
    assert result == first - reverse_polygon_holes(second)
    assert result == reverse_compound_coordinates(
            reverse_polygon_coordinates(first)
            - reverse_polygon_coordinates(second)
    )
