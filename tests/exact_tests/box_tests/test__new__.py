from hypothesis import given

from rene.exact import Box
from tests.hints import Scalar

from . import strategies


@given(strategies.boxes_limits)
def test_basic(limits: tuple[Scalar, Scalar, Scalar, Scalar]) -> None:
    min_x, max_x, min_y, max_y = limits

    result = Box(min_x, max_x, min_y, max_y)

    assert isinstance(result, Box)
    assert result.min_x == min_x
    assert result.max_x == max_x
    assert result.min_y == min_y
    assert result.max_y == max_y
