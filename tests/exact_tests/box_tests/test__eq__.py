from hypothesis import given

from rene.exact import Box
from tests.utils import equivalence, implication, reverse_box_coordinates

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
    assert equivalence(first == second, first == second)


@given(strategies.boxes, strategies.boxes)
def test_reversals(first: Box, second: Box) -> None:
    assert equivalence(
        first == second,
        reverse_box_coordinates(first) == reverse_box_coordinates(second),
    )
