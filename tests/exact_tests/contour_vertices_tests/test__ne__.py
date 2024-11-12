import typing as t

from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence

from . import strategies


@given(strategies.contours_vertices)
def test_irreflexivity(vertices: t.Sequence[Point]) -> None:
    assert vertices == vertices


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_symmetry(first: t.Sequence[Point], second: t.Sequence[Point]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_equivalents(
    first: t.Sequence[Point], second: t.Sequence[Point]
) -> None:
    assert equivalence(first != second, first != second)
