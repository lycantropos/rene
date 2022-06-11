from hypothesis import given

from rene.exact import Contour
from tests.utils import implication
from . import strategies


@given(strategies.contours)
def test_determinism(contour: Contour) -> None:
    result = hash(contour)

    assert result == hash(contour)


@given(strategies.contours, strategies.contours)
def test_preserving_equality(first: Contour, second: Contour) -> None:
    assert implication(first == second, hash(first) == hash(second))
