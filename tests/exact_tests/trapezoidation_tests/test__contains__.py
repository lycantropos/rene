from hypothesis import given

from rene.exact import (Point,
                        Trapezoidation)
from . import strategies


@given(strategies.trapezoidations, strategies.points)
def test_basic(trapezoidation: Trapezoidation, point: Point) -> None:
    result = point in trapezoidation

    assert isinstance(result, bool)
