from collections.abc import Sequence

from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence

from . import strategies


@given(strategies.contours_vertices)
def test_irreflexivity(vertices: Sequence[Point]) -> None:
    assert vertices == vertices


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_symmetry(first: Sequence[Point], second: Sequence[Point]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_equivalents(first: Sequence[Point], second: Sequence[Point]) -> None:
    assert equivalence(first != second, first != second)
