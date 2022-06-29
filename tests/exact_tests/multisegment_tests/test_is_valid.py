from hypothesis import given

from rene.exact import Multisegment
from tests.utils import (equivalence, reverse_multisegment,
                         rotate_multisegment)
from . import strategies


@given(strategies.multisegments_like)
def test_basic(multisegment: Multisegment) -> None:
    assert isinstance(multisegment.is_valid(), bool)


@given(strategies.multisegments, strategies.non_zero_integers)
def test_vertices_rotations(multisegment: Multisegment, offset: int) -> None:
    assert equivalence(multisegment.is_valid(),
                       rotate_multisegment(multisegment, offset).is_valid())


@given(strategies.multisegments)
def test_vertices_reversal(multisegment: Multisegment) -> None:
    assert equivalence(multisegment.is_valid(),
                       reverse_multisegment(multisegment).is_valid())
