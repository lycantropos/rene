from hypothesis import given

from rene import Location
from rene.exact import (Point,
                        Trapezoidation)
from . import strategies


@given(strategies.trapezoidations, strategies.points)
def test_basic(trapezoidation: Trapezoidation, point: Point) -> None:
    result = trapezoidation.locate(point)

    assert isinstance(result, Location)
