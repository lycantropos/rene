import sys

from hypothesis import given

from rene.exact import Multisegment
from . import strategies


@given(strategies.multisegments)
def test_round_trip(multisegment: Multisegment) -> None:
    result = repr(multisegment)

    assert eval(result, sys.modules) == multisegment
