from hypothesis import given

from rene.exact import Box
from tests.utils import (equivalence,
                         implication)
from . import strategies


@given(strategies.boxes)
def test_reflexivity(box: Box) -> None:
    assert box == box


@given(strategies.boxes, strategies.boxes)
def test_symmetry(first: Box, second: Box) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.boxes, strategies.boxes, strategies.boxes)
def test_transitivity(first: Box, second: Box, third: Box) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.boxes, strategies.boxes)
def test_alternatives(first: Box, second: Box) -> None:
    assert equivalence(first == second, not first != second)
