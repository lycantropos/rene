from collections.abc import Sequence

from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence, implication, reverse_sequence

from . import strategies


@given(strategies.contours_vertices)
def test_reflexivity(vertices: Sequence[Point]) -> None:
    assert vertices == vertices


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_symmetry(first: Sequence[Point], second: Sequence[Point]) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.contours_vertices,
    strategies.contours_vertices,
    strategies.contours_vertices,
)
def test_transitivity(
    first: Sequence[Point], second: Sequence[Point], third: Sequence[Point]
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_alternatives(first: Sequence[Point], second: Sequence[Point]) -> None:
    assert equivalence(first == second, first == second)


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_reversals(first: Sequence[Point], second: Sequence[Point]) -> None:
    assert equivalence(
        first == second, reverse_sequence(first) == reverse_sequence(second)
    )
