import sys

from hypothesis import given

from rene.exact import Contour
from . import strategies


@given(strategies.contours)
def test_round_trip(contour: Contour) -> None:
    result = repr(contour)

    assert eval(result, sys.modules) == contour
