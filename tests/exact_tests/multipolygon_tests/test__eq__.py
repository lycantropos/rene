from hypothesis import given

from rene.exact import Multipolygon
from tests.utils import (equivalence,
                         implication,
                         reverse_multipolygon_polygons,
                         rotate_multipolygon)
from . import strategies


@given(strategies.multipolygons)
def test_reflexivity(multipolygon: Multipolygon) -> None:
    assert multipolygon == multipolygon


@given(strategies.multipolygons, strategies.multipolygons)
def test_symmetry(first: Multipolygon, second: Multipolygon) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.multipolygons, strategies.multipolygons, strategies.multipolygons)
def test_transitivity(first: Multipolygon, second: Multipolygon, third: Multipolygon) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.multipolygons, strategies.multipolygons)
def test_alternatives(first: Multipolygon, second: Multipolygon) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.multipolygons)
def test_polygons_reversal(multipolygon: Multipolygon) -> None:
    assert multipolygon == reverse_multipolygon_polygons(multipolygon)


@given(strategies.multipolygons, strategies.non_zero_integers)
def test_polygons_rotations(multipolygon: Multipolygon, offset: int) -> None:
    assert multipolygon == rotate_multipolygon(multipolygon, offset)


@given(strategies.multipolygons, strategies.non_zero_integers)
def test_polygons_rotations_of_reversal(multipolygon: Multipolygon, offset: int) -> None:
    assert multipolygon == rotate_multipolygon(reverse_multipolygon_polygons(multipolygon), offset)
