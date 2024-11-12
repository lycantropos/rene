from collections.abc import Sequence

from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence

from . import strategies


@given(strategies.contours_vertices, strategies.points)
def test_basic(vertices: Sequence[Point], point: Point) -> None:
    result = point in vertices

    assert isinstance(result, bool)


@given(strategies.contours_vertices, strategies.points)
def test_alternatives(vertices: Sequence[Point], point: Point) -> None:
    result = point in vertices

    assert equivalence(result, point in iter(vertices))
    assert equivalence(result, point in list(vertices))
    assert equivalence(result, point in tuple(vertices))
