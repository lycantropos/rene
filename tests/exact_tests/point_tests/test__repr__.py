import sys

from hypothesis import given

from rene.exact import Point
from . import strategies


@given(strategies.points)
def test_round_trip(point: Point) -> None:
    result = repr(point)

    assert eval(result, {**vars(sys.modules['rene.exact']),
                         **vars(sys.modules['rithm.fraction']),
                         **vars(sys.modules['rithm.integer'])}) == point
