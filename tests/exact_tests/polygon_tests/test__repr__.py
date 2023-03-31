from hypothesis import given
from rithm import (fraction,
                   integer)

from rene import exact
from rene.exact import Polygon
from . import strategies


@given(strategies.polygons)
def test_round_trip(polygon: Polygon) -> None:
    result = repr(polygon)

    assert eval(result,
                {**vars(exact), **vars(fraction), **vars(integer)}) == polygon
