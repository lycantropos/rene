from hypothesis import given

from rene.exact import Contour
from tests.utils import (equivalence,
                         implication,
                         reverse_contour,
                         reverse_contour_coordinates,
                         rotate_contour)
from . import strategies


@given(strategies.contours)
def test_reflexivity(contour: Contour) -> None:
    assert contour == contour


@given(strategies.contours, strategies.contours)
def test_symmetry(first: Contour, second: Contour) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.contours, strategies.contours, strategies.contours)
def test_transitivity(first: Contour, second: Contour, third: Contour) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.contours, strategies.contours)
def test_alternatives(first: Contour, second: Contour) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.contours, strategies.contours)
def test_reversals(first: Contour, second: Contour) -> None:
    assert equivalence(first == second,
                       reverse_contour(first) == reverse_contour(second))
    assert equivalence(first == second,
                       (reverse_contour_coordinates(first)
                        == reverse_contour_coordinates(second)))


@given(strategies.contours)
def test_vertices_reversal(contour: Contour) -> None:
    assert contour == reverse_contour(contour)


@given(strategies.contours, strategies.non_zero_integers)
def test_vertices_rotations(contour: Contour, offset: int) -> None:
    assert contour == rotate_contour(contour, offset)


@given(strategies.contours, strategies.non_zero_integers)
def test_vertices_rotations_of_reversal(contour: Contour, offset: int) -> None:
    assert contour == rotate_contour(reverse_contour(contour), offset)
