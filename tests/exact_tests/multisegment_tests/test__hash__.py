from hypothesis import given

from rene.exact import Multisegment
from tests.utils import implication

from . import strategies


@given(strategies.multisegments)
def test_determinism(multisegment: Multisegment) -> None:
    result = hash(multisegment)

    assert result == hash(multisegment)


@given(strategies.multisegments, strategies.multisegments)
def test_preserving_equality(
    first: Multisegment, second: Multisegment
) -> None:
    assert implication(first == second, hash(first) == hash(second))
