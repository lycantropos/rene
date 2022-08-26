import sys

from hypothesis import given

from rene.exact import Multipolygon
from . import strategies


@given(strategies.multipolygons)
def test_round_trip(multipolygon: Multipolygon) -> None:
    result = repr(multipolygon)

    assert eval(result, sys.modules) == multipolygon
