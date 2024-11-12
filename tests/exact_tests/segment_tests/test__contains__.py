from hypothesis import given

from rene.enums import Location
from rene.exact import Point, Segment
from tests.utils import equivalence

from . import strategies


@given(strategies.segments, strategies.points)
def test_basic(segment: Segment, point: Point) -> None:
    result = point in segment

    assert isinstance(result, bool)


@given(strategies.segments, strategies.points)
def test_alternatives(segment: Segment, point: Point) -> None:
    result = point in segment

    assert equivalence(result, segment.locate(point) is not Location.EXTERIOR)
