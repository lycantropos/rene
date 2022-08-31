from hypothesis import given

from rene.exact import Box
from . import strategies


@given(strategies.boxes_like)
def test_basic(box: Box) -> None:
    assert isinstance(box.is_valid(), bool)
