from hypothesis import given

from rene.enums import Location
from rene.exact import Contour, Point
from tests.utils import equivalence

from . import strategies


@given(strategies.contours, strategies.points)
def test_basic(contour: Contour, point: Point) -> None:
    result = point in contour

    assert isinstance(result, bool)


@given(strategies.contours, strategies.points)
def test_alternatives(contour: Contour, point: Point) -> None:
    result = point in contour

    assert equivalence(result, contour.locate(point) is not Location.EXTERIOR)
