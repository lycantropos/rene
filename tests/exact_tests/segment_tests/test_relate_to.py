from hypothesis import given

from rene import Relation
from rene.exact import Segment
from tests.utils import reverse_segment_coordinates, reverse_segment_endpoints
from . import strategies


@given(strategies.segments, strategies.segments)
def test_basic(first: Segment, second: Segment) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.segments, strategies.segments)
def test_reversals(first: Segment, second: Segment) -> None:
    assert (first.relate_to(second)
            is reverse_segment_endpoints(first).relate_to(
                    reverse_segment_endpoints(second)
            ))
    assert (first.relate_to(second)
            is reverse_segment_coordinates(first).relate_to(
                    reverse_segment_coordinates(second)
            ))
    assert first.relate_to(second) is second.relate_to(first).complement
