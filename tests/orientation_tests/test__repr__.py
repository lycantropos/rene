from hypothesis import given

from rene import Orientation

from . import strategies


@given(strategies.orientations)
def test_round_trip(orientation: Orientation) -> None:
    result = repr(orientation)

    assert eval(result, {Orientation.__qualname__: Orientation}) is orientation
