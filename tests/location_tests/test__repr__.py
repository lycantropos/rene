from hypothesis import given

from rene import Location
from . import strategies


@given(strategies.locations)
def test_round_trip(location: Location) -> None:
    result = repr(location)

    assert eval(result, {Location.__qualname__: Location}) is location
