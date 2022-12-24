from hypothesis import given

from rene import Relation
from rene.exact import Box
from tests.utils import reverse_box_coordinates
from . import strategies


@given(strategies.boxes, strategies.boxes)
def test_basic(first: Box, second: Box) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.boxes, strategies.boxes)
def test_reversals(first: Box, second: Box) -> None:
    assert first.relate_to(second) is second.relate_to(first).complement
    assert (first.relate_to(second)
            is reverse_box_coordinates(first).relate_to(
                    reverse_box_coordinates(second)
            ))
