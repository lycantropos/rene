from hypothesis import given
from rithm import (fraction,
                   integer)

from rene import exact
from rene.exact import Contour
from . import strategies


@given(strategies.contours)
def test_round_trip(contour: Contour) -> None:
    result = repr(contour)

    assert eval(result,
                {**vars(exact), **vars(fraction), **vars(integer)}) == contour
