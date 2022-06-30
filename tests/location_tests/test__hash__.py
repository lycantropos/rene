from hypothesis import given

from rene import Location
from tests.utils import implication
from . import strategies


@given(strategies.locations)
def test_determinism(location: Location) -> None:
    result = hash(location)

    assert result == hash(location)


@given(strategies.locations, strategies.locations)
def test_preserving_equality(first: Location, second: Location) -> None:
    assert implication(first == second, hash(first) == hash(second))
