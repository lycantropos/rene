from hypothesis import given

from rene import Relation
from rene.exact import Box
from . import strategies


@given(strategies.boxes, strategies.boxes)
def test_basic(first: Box, second: Box) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.boxes, strategies.boxes)
def test_operands_swap(first: Box, second: Box) -> None:
    assert first.relate_to(second) is second.relate_to(first).complement
