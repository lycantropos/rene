import sys

from hypothesis import given

from rene.exact import Segment
from . import strategies


@given(strategies.segments)
def test_round_trip(segment: Segment) -> None:
    result = repr(segment)

    assert eval(result, {**vars(sys.modules['rene.exact']),
                         **vars(sys.modules['rithm.fraction']),
                         **vars(sys.modules['rithm.integer'])}) == segment
