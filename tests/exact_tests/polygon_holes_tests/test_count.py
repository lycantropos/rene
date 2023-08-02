import typing as t

from hypothesis import given

from rene.exact import Contour
from tests.utils import equivalence
from . import strategies


@given(strategies.polygons_holes, strategies.contours)
def test_basic(holes: t.Sequence[Contour], contour: Contour) -> None:
    result = holes.count(contour)

    assert isinstance(result, int)
    assert result in range(max(len(holes), 1))
    assert equivalence(result == 0, contour not in holes)


@given(strategies.polygons_holes, strategies.contours)
def test_alternatives(holes: t.Sequence[Contour], contour: Contour) -> None:
    result = holes.count(contour)

    assert result == list(holes).count(contour)
    assert result == tuple(holes).count(contour)
