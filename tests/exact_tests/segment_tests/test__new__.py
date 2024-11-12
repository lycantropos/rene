from typing import Tuple

from hypothesis import given

from rene.exact import Point, Segment

from . import strategies


@given(strategies.segments_endpoints)
def test_basic(endpoints: Tuple[Point, Point]) -> None:
    start, end = endpoints

    result = Segment(start, end)

    assert isinstance(result, Segment)
    assert isinstance(result.start, Point)
    assert isinstance(result.end, Point)
    assert result.start == start
    assert result.end == end
