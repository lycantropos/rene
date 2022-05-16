from typing import Sequence

import pytest
from hypothesis import given

from rene.exact import (Contour,
                        Point)
from . import strategies


@given(strategies.contours_vertices)
def test_basic(vertices: Sequence[Point]) -> None:
    result = Contour(vertices)

    assert isinstance(result, Contour)
    assert result.vertices == vertices


@given(strategies.invalid_count_contours_vertices)
def test_invalid_vertices_count(vertices: Sequence[Point]) -> None:
    with pytest.raises(ValueError):
        Contour(vertices)
