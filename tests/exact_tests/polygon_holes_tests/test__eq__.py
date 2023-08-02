import typing as t

from hypothesis import given

from rene.exact import Contour
from tests.utils import (equivalence,
                         implication,
                         reverse_sequence)
from . import strategies


@given(strategies.polygons_holes)
def test_reflexivity(holes: t.Sequence[Contour]) -> None:
    assert holes == holes


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_symmetry(first: t.Sequence[Contour],
                  second: t.Sequence[Contour]) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.polygons_holes, strategies.polygons_holes,
       strategies.polygons_holes)
def test_transitivity(first: t.Sequence[Contour],
                      second: t.Sequence[Contour],
                      third: t.Sequence[Contour]) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_alternatives(first: t.Sequence[Contour],
                      second: t.Sequence[Contour]) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.polygons_holes, strategies.polygons_holes)
def test_reversals(first: t.Sequence[Contour],
                   second: t.Sequence[Contour]) -> None:
    assert equivalence(first == second,
                       reverse_sequence(first) == reverse_sequence(second))
