from hypothesis import given
from rithm import (fraction,
                   integer)

from rene import exact
from rene.exact import Multisegment
from . import strategies


@given(strategies.multisegments)
def test_round_trip(multisegment: Multisegment) -> None:
    result = repr(multisegment)

    assert eval(
            result, {**vars(exact), **vars(fraction), **vars(integer)}
    ) == multisegment
