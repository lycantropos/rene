from collections.abc import Sequence

from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence

from . import strategies


@given(strategies.polygons_holes, strategies.contours)
def test_basic(holes: Sequence[Contour], contour: Contour) -> None:
    result = contour in holes

    assert isinstance(result, bool)


@given(strategies.polygons_holes, strategies.contours)
def test_alternatives(holes: Sequence[Contour], contour: Contour) -> None:
    result = contour in holes

    assert equivalence(result, contour in iter(holes))
    assert equivalence(result, contour in list(holes))
    assert equivalence(result, contour in tuple(holes))
