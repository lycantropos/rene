from hypothesis import given
from rithm import fraction, integer

from rene import exact
from rene.exact import Segment

from . import strategies


@given(strategies.segments)
def test_round_trip(segment: Segment) -> None:
    result = repr(segment)

    assert (
        eval(result, {**vars(exact), **vars(fraction), **vars(integer)})
        == segment
    )
