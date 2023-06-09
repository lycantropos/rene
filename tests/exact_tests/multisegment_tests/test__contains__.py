from hypothesis import given

from rene import Location
from rene.exact import (Multisegment,
                        Point)
from tests.utils import equivalence
from . import strategies


@given(strategies.multisegments, strategies.points)
def test_basic(multisegment: Multisegment, point: Point) -> None:
    result = point in multisegment

    assert isinstance(result, bool)


@given(strategies.multisegments, strategies.points)
def test_alternatives(multisegment: Multisegment, point: Point) -> None:
    result = point in multisegment

    assert equivalence(result,
                       multisegment.locate(point) is not Location.EXTERIOR)
