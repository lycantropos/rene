from hypothesis import given

from rene.exact import Polygon
from tests.utils import (equivalence,
                         implication,
                         reverse_polygon_holes,
                         rotate_each_polygon_hole,
                         rotate_polygon_border,
                         rotate_polygon_holes)
from . import strategies


@given(strategies.polygons)
def test_reflexivity(polygon: Polygon) -> None:
    assert polygon == polygon


@given(strategies.polygons, strategies.polygons)
def test_symmetry(first: Polygon, second: Polygon) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.polygons, strategies.polygons, strategies.polygons)
def test_transitivity(first: Polygon, second: Polygon, third: Polygon) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.polygons, strategies.polygons)
def test_alternatives(first: Polygon, second: Polygon) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.polygons, strategies.non_zero_integers)
def test_border_rotations(polygon: Polygon, offset: int) -> None:
    assert polygon == rotate_polygon_border(polygon, offset)


@given(strategies.polygons, strategies.non_zero_integers)
def test_each_hole_rotations(polygon: Polygon, offset: int) -> None:
    assert polygon == rotate_each_polygon_hole(polygon, offset)


@given(strategies.polygons)
def test_holes_reversal(polygon: Polygon) -> None:
    assert polygon == reverse_polygon_holes(polygon)


@given(strategies.polygons, strategies.non_zero_integers)
def test_holes_rotations(polygon: Polygon, offset: int) -> None:
    assert polygon == rotate_polygon_holes(polygon, offset)
