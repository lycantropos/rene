import sys

from hypothesis import given

from rene.exact import Polygon
from . import strategies


@given(strategies.polygons)
def test_round_trip(polygon: Polygon) -> None:
    result = repr(polygon)

    assert eval(result, {**vars(sys.modules['rene.exact']),
                         **vars(sys.modules['rithm.fraction']),
                         **vars(sys.modules['rithm.integer'])}) == polygon
