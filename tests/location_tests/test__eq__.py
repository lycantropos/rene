from hypothesis import given

from rene import Location
from tests.utils import equivalence, implication

from . import strategies


@given(strategies.locations)
def test_reflexivity(location: Location) -> None:
    assert location == location


@given(strategies.locations, strategies.locations)
def test_symmetry(first: Location, second: Location) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.locations, strategies.locations, strategies.locations)
def test_transitivity(
    first: Location, second: Location, third: Location
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.locations, strategies.locations)
def test_alternatives(first: Location, second: Location) -> None:
    assert equivalence(first == second, first == second)
