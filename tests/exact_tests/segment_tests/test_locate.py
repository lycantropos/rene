from hypothesis import given

from rene import Location
from rene.exact import (Point,
                        Segment)
from tests.utils import (reverse_point_coordinates,
                         reverse_segment_coordinates,
                         reverse_segment_endpoints)
from . import strategies


@given(strategies.segments, strategies.points)
def test_basic(segment: Segment, point: Point) -> None:
    result = segment.locate(point)

    assert isinstance(result, Location)


@given(strategies.segments)
def test_endpoints(segment: Segment) -> None:
    assert segment.locate(segment.start) is Location.BOUNDARY
    assert segment.locate(segment.end) is Location.BOUNDARY


@given(strategies.segments, strategies.points)
def test_reversals(segment: Segment, point: Point) -> None:
    assert (segment.locate(point)
            is reverse_segment_endpoints(segment).locate(point))
    assert (segment.locate(point)
            is reverse_segment_coordinates(segment).locate(
                    reverse_point_coordinates(point)
            ))
