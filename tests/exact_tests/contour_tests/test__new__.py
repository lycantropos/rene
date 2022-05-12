from typing import Sequence

from hypothesis import given

from rene.exact import (Contour,
                        Point)
from . import strategies


@given(strategies.contours_vertices)
def test_basic(vertices: Sequence[Point]) -> None:
    result = Contour(vertices)

    assert isinstance(result, Contour)
    assert result.vertices == vertices
