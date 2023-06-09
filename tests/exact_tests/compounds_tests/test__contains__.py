from hypothesis import given

from rene import Location
from rene.exact import Point
from tests.utils import (Compound,
                         equivalence,
                         reverse_compound_coordinates,
                         reverse_point_coordinates)
from . import strategies


@given(strategies.compounds, strategies.points)
def test_basic(compound: Compound, point: Point) -> None:
    result = point in compound

    assert isinstance(result, bool)


@given(strategies.compounds, strategies.points)
def test_alternatives(compound: Compound, point: Point) -> None:
    result = point in compound

    assert equivalence(result, compound.locate(point) is not Location.EXTERIOR)


@given(strategies.compounds, strategies.points)
def test_reversals(compound: Compound, point: Point) -> None:
    result = point in compound

    assert equivalence(result, (reverse_point_coordinates(point)
                                in reverse_compound_coordinates(compound)))
