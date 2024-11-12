from hypothesis import given

from rene import Location
from rene.exact import Point, Trapezoidation
from tests.utils import equivalence

from . import strategies


@given(strategies.trapezoidations, strategies.points)
def test_basic(trapezoidation: Trapezoidation, point: Point) -> None:
    result = point in trapezoidation

    assert isinstance(result, bool)


@given(strategies.trapezoidations, strategies.points)
def test_alternatives(trapezoidation: Trapezoidation, point: Point) -> None:
    result = point in trapezoidation

    assert equivalence(
        result, trapezoidation.locate(point) is not Location.EXTERIOR
    )
