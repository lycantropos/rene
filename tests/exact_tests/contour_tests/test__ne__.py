from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence

from . import strategies


@given(strategies.contours)
def test_irreflexivity(contour: Contour) -> None:
    assert contour == contour


@given(strategies.contours, strategies.contours)
def test_symmetry(first: Contour, second: Contour) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.contours, strategies.contours)
def test_equivalents(first: Contour, second: Contour) -> None:
    assert equivalence(first != second, first != second)
