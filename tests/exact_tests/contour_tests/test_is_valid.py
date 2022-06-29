from hypothesis import given

from rene.exact import Contour
from tests.utils import (equivalence,
                         reverse_contour,
                         rotate_contour)
from . import strategies


@given(strategies.contours_like)
def test_basic(contour: Contour) -> None:
    assert isinstance(contour.is_valid(), bool)


@given(strategies.contours, strategies.non_zero_integers)
def test_vertices_rotations(contour: Contour, offset: int) -> None:
    assert equivalence(contour.is_valid(),
                       rotate_contour(contour, offset).is_valid())


@given(strategies.contours)
def test_vertices_reversal(contour: Contour) -> None:
    assert equivalence(contour.is_valid(), reverse_contour(contour).is_valid())
