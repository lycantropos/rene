import typing as t

from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence

from . import strategies


@given(strategies.polygons_holes, strategies.contours)
def test_basic(holes: t.Sequence[Contour], contour: Contour) -> None:
    result = contour in holes

    assert isinstance(result, bool)


@given(strategies.polygons_holes, strategies.contours)
def test_alternatives(holes: t.Sequence[Contour], contour: Contour) -> None:
    result = contour in holes

    assert equivalence(result, contour in iter(holes))
    assert equivalence(result, contour in list(holes))
    assert equivalence(result, contour in tuple(holes))
