from collections.abc import Sequence

from hypothesis import given

from rene.exact import Point
from tests.utils import equivalence

from . import strategies


@given(strategies.contours_vertices, strategies.points)
def test_basic(vertices: Sequence[Point], point: Point) -> None:
    result = vertices.count(point)

    assert isinstance(result, int)
    assert result in range(len(vertices))
    assert equivalence(result == 0, point not in vertices)


@given(strategies.contours_vertices, strategies.points)
def test_alternatives(vertices: Sequence[Point], point: Point) -> None:
    result = vertices.count(point)

    assert result == list(vertices).count(point)
    assert result == tuple(vertices).count(point)
