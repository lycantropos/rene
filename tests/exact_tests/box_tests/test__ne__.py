from hypothesis import given

from rene.exact import Box
from tests.utils import equivalence

from . import strategies


@given(strategies.boxes)
def test_irreflexivity(box: Box) -> None:
    assert box == box


@given(strategies.boxes, strategies.boxes)
def test_symmetry(first: Box, second: Box) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.boxes, strategies.boxes)
def test_equivalents(first: Box, second: Box) -> None:
    assert equivalence(first != second, first != second)
