from hypothesis import given

from rene.enums import Orientation
from rene.exact import Contour
from tests.utils import rotate_contour

from . import strategies


@given(strategies.contours)
def test_basic(contour: Contour) -> None:
    assert isinstance(contour.orientation, Orientation)


@given(strategies.contours, strategies.non_zero_integers)
def test_vertices_rotations(contour: Contour, offset: int) -> None:
    assert contour.orientation is rotate_contour(contour, offset).orientation
