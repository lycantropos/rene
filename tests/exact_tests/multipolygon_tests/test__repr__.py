from hypothesis import given
from rithm import fraction, integer

from rene import exact
from rene.exact import Multipolygon

from . import strategies


@given(strategies.multipolygons)
def test_round_trip(multipolygon: Multipolygon) -> None:
    result = repr(multipolygon)

    assert (
        eval(result, {**vars(exact), **vars(fraction), **vars(integer)})
        == multipolygon
    )
