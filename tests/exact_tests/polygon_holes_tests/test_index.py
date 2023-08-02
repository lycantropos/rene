import typing as t

import pytest
from hypothesis import given

from rene.exact import Contour
from . import strategies


@given(strategies.polygons_holes, strategies.contours)
def test_basic_default_range(holes: t.Sequence[Contour],
                             contour: Contour) -> None:
    try:
        result = holes.index(contour)
    except ValueError:
        assert contour not in holes
    else:
        assert isinstance(result, int)
        assert result in range(len(holes))
        assert holes[result] == contour


@given(strategies.polygons_holes, strategies.contours,
       strategies.indices, strategies.indices)
def test_basic_custom_range(holes: t.Sequence[Contour],
                            contour: Contour,
                            start: int,
                            stop: int) -> None:
    try:
        result = holes.index(contour, start, stop)
    except ValueError:
        assert contour not in holes[start:stop]
    else:
        assert isinstance(result, int)
        assert result in range(start, stop)
        assert holes[result] == contour


@given(strategies.polygons_holes, strategies.contours)
def test_alternatives_default_range(holes: t.Sequence[Contour],
                                    contour: Contour) -> None:

    try:
        result = holes.index(contour)
    except ValueError:
        with pytest.raises(ValueError):
            list(holes).index(contour)
        with pytest.raises(ValueError):
            tuple(holes).index(contour)
    else:
        assert result == list(holes).index(contour)
        assert result == tuple(holes).index(contour)


@given(strategies.polygons_holes, strategies.contours,
       strategies.indices, strategies.indices)
def test_alternatives_custom_range(holes: t.Sequence[Contour],
                                   contour: Contour,
                                   start: int,
                                   stop: int) -> None:

    try:
        result = holes.index(contour, start, stop)
    except ValueError:
        with pytest.raises(ValueError):
            list(holes).index(contour, start, stop)
    else:
        assert result == list(holes).index(contour, start, stop)
        assert result == tuple(holes).index(contour, start, stop)
