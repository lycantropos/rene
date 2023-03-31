from hypothesis import given
from rithm import (fraction,
                   integer)

from rene import exact
from rene.exact import Point
from . import strategies


@given(strategies.points)
def test_round_trip(point: Point) -> None:
    result = repr(point)

    assert eval(result,
                {**vars(exact), **vars(fraction), **vars(integer)}) == point
