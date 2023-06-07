from hypothesis import given

from rene.exact import (Box,
                        Segment)
from tests.utils import (is_segment_inside_box,
                         reverse_box_coordinates,
                         reverse_segment_coordinates)
from . import strategies


@given(strategies.segments)
def test_basic(segment: Segment) -> None:
    result = segment.bounding_box

    assert isinstance(result, Box)


@given(strategies.segments)
def test_relations(segment: Segment) -> None:
    result = segment.bounding_box

    assert is_segment_inside_box(segment, result)


@given(strategies.segments)
def test_reversals(segment: Segment) -> None:
    assert (reverse_box_coordinates(segment.bounding_box)
            == reverse_segment_coordinates(segment).bounding_box)
