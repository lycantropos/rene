import sys

from hypothesis import given

from rene.exact import Box
from . import strategies


@given(strategies.boxes)
def test_round_trip(box: Box) -> None:
    result = repr(box)

    assert eval(result, sys.modules) == box
