from hypothesis import given
from rithm import (fraction,
                   integer)

from rene import exact
from rene.exact import Box
from . import strategies


@given(strategies.boxes)
def test_round_trip(box: Box) -> None:
    result = repr(box)

    assert eval(result,
                {**vars(exact), **vars(fraction), **vars(integer)}) == box
