import typing as t

from hypothesis import given

from rene.exact import Point
from tests.utils import implication

from . import strategies


@given(strategies.contours_vertices)
def test_determinism(vertices: t.Sequence[Point]) -> None:
    result = hash(vertices)

    assert result == hash(vertices)


@given(strategies.contours_vertices, strategies.contours_vertices)
def test_preserving_equality(
    first: t.Sequence[Point], second: t.Sequence[Point]
) -> None:
    assert implication(first == second, hash(first) == hash(second))
