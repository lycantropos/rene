import typing as t

import pytest
from hypothesis import given

from rene.exact import Point
from . import strategies


@given(strategies.contours_vertices, strategies.points)
def test_basic_default_range(vertices: t.Sequence[Point],
                             point: Point) -> None:
    try:
        result = vertices.index(point)
    except ValueError:
        assert point not in vertices
    else:
        assert isinstance(result, int)
        assert result in range(len(vertices))
        assert vertices[result] == point


@given(strategies.contours_vertices, strategies.points,
       strategies.indices, strategies.indices)
def test_basic_custom_range(vertices: t.Sequence[Point],
                            point: Point,
                            start: int,
                            stop: int) -> None:
    try:
        result = vertices.index(point, start, stop)
    except ValueError:
        assert point not in vertices[start:stop]
    else:
        assert isinstance(result, int)
        assert result in range(start, stop)
        assert vertices[result] == point


@given(strategies.contours_vertices, strategies.points)
def test_alternatives_default_range(vertices: t.Sequence[Point],
                                    point: Point) -> None:

    try:
        result = vertices.index(point)
    except ValueError:
        with pytest.raises(ValueError):
            list(vertices).index(point)
        with pytest.raises(ValueError):
            tuple(vertices).index(point)
    else:
        assert result == list(vertices).index(point)
        assert result == tuple(vertices).index(point)


@given(strategies.contours_vertices, strategies.points,
       strategies.indices, strategies.indices)
def test_alternatives_custom_range(vertices: t.Sequence[Point],
                                   point: Point,
                                   start: int,
                                   stop: int) -> None:

    try:
        result = vertices.index(point, start, stop)
    except ValueError:
        with pytest.raises(ValueError):
            list(vertices).index(point, start, stop)
    else:
        assert result == list(vertices).index(point, start, stop)
        assert result == tuple(vertices).index(point, start, stop)
