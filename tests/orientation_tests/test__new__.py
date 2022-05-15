from hypothesis import given

from rene import Orientation
from . import strategies


@given(strategies.orientations_values)
def test_basic(value: int) -> None:
    result = Orientation(value)

    assert isinstance(result, Orientation)
    assert result.value == value
