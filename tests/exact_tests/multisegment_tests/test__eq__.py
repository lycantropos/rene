from hypothesis import given

from rene.exact import Multisegment
from tests.utils import (equivalence,
                         implication,
                         reverse_multisegment,
                         reverse_multisegment_coordinates,
                         rotate_multisegment)
from . import strategies


@given(strategies.multisegments)
def test_reflexivity(multisegment: Multisegment) -> None:
    assert multisegment == multisegment


@given(strategies.multisegments, strategies.multisegments)
def test_symmetry(first: Multisegment, second: Multisegment) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.multisegments, strategies.multisegments,
       strategies.multisegments)
def test_transitivity(first: Multisegment,
                      second: Multisegment,
                      third: Multisegment) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.multisegments, strategies.multisegments)
def test_alternatives(first: Multisegment, second: Multisegment) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.multisegments, strategies.multisegments)
def test_reversals(first: Multisegment, second: Multisegment) -> None:
    assert equivalence(
            first == second,
            reverse_multisegment(first) == reverse_multisegment(second)
    )
    assert equivalence(first == second,
                       (reverse_multisegment_coordinates(first)
                        == reverse_multisegment_coordinates(second)))


@given(strategies.multisegments)
def test_vertices_reversal(multisegment: Multisegment) -> None:
    assert multisegment == reverse_multisegment(multisegment)


@given(strategies.multisegments, strategies.non_zero_integers)
def test_vertices_rotations(multisegment: Multisegment, offset: int) -> None:
    assert multisegment == rotate_multisegment(multisegment, offset)


@given(strategies.multisegments, strategies.non_zero_integers)
def test_vertices_rotations_of_reversal(multisegment: Multisegment, offset: int
                                        ) -> None:
    assert (multisegment
            == rotate_multisegment(reverse_multisegment(multisegment), offset))
