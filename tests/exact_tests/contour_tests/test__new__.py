from typing import Sequence

import pytest
from hypothesis import given

from rene.exact import Contour, Point
from tests.utils import are_sequences_equivalent

from . import strategies


@given(strategies.contours_vertices)
def test_basic(vertices: Sequence[Point]) -> None:
    result = Contour(vertices)

    assert isinstance(result, Contour)
    assert are_sequences_equivalent(result.vertices, vertices)


@given(strategies.invalid_count_contours_vertices)
def test_invalid_vertices_count(vertices: Sequence[Point]) -> None:
    with pytest.raises(ValueError):
        Contour(vertices)
