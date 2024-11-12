from hypothesis import given

from rene.exact import Multisegment
from tests.utils import equivalence

from . import strategies


@given(strategies.multisegments)
def test_irreflexivity(multisegment: Multisegment) -> None:
    assert multisegment == multisegment


@given(strategies.multisegments, strategies.multisegments)
def test_symmetry(first: Multisegment, second: Multisegment) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multisegments, strategies.multisegments)
def test_equivalents(first: Multisegment, second: Multisegment) -> None:
    assert equivalence(first != second, first != second)
