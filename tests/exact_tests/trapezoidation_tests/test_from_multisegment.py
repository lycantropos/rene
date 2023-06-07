from hypothesis import given

from rene.exact import (Multisegment,
                        Trapezoidation)
from . import strategies


@given(strategies.multisegments)
def test_basic(multisegment: Multisegment) -> None:
    result = Trapezoidation.from_multisegment(multisegment)

    assert isinstance(result, Trapezoidation)
