from hypothesis import given

from rene.enums import Location
from rene.exact import Point
from tests.exact_tests.hints import Compound
from tests.utils import reverse_compound_coordinates, reverse_point_coordinates

from . import strategies


@given(strategies.compounds, strategies.points)
def test_basic(compound: Compound, point: Point) -> None:
    result = compound.locate(point)

    assert isinstance(result, Location)


@given(strategies.compounds, strategies.points)
def test_reversals(compound: Compound, point: Point) -> None:
    assert compound.locate(point) is reverse_compound_coordinates(
        compound
    ).locate(reverse_point_coordinates(point))
