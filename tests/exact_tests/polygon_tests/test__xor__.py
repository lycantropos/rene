from hypothesis import given

from rene.exact import (Empty,
                        Multipolygon,
                        Polygon)
from tests.utils import (Compound,
                         reverse_compound_coordinates,
                         reverse_polygon_coordinates,
                         reverse_polygon_holes)
from . import strategies


@given(strategies.polygons, strategies.compounds)
def test_basic(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert isinstance(result, (Empty, Multipolygon, Polygon))


@given(strategies.polygons)
def test_self_inverse(polygon: Polygon) -> None:
    result = polygon ^ polygon

    assert isinstance(result, Empty)


@given(strategies.polygons, strategies.compounds)
def test_commutativity(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert result == second ^ first


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_associativity(first: Polygon,
                       second: Compound,
                       third: Compound) -> None:
    assert (first ^ second) ^ third == first ^ (second ^ third)


@given(strategies.polygons, strategies.compounds, strategies.compounds)
def test_repeated(first: Polygon, second: Compound, third: Compound) -> None:
    assert (first ^ second) ^ (second ^ third) == first ^ third


@given(strategies.polygons, strategies.compounds)
def test_alternatives(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert result == (first - second) | (second - first)
    assert result == (first | second) - (second & first)


@given(strategies.polygons, strategies.compounds)
def test_reversals(first: Polygon, second: Compound) -> None:
    result = first ^ second

    assert result == reverse_polygon_holes(first) ^ second
    assert result == first ^ reverse_polygon_holes(second)
    assert result == reverse_compound_coordinates(
            reverse_polygon_coordinates(first)
            ^ reverse_polygon_coordinates(second)
    )
