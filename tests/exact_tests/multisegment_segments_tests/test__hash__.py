from collections.abc import Sequence

from hypothesis import given

from rene.exact import Segment
from tests.utils import implication

from . import strategies


@given(strategies.multisegments_segments)
def test_determinism(segments: Sequence[Segment]) -> None:
    result = hash(segments)

    assert result == hash(segments)


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_preserving_equality(
    first: Sequence[Segment], second: Sequence[Segment]
) -> None:
    assert implication(first == second, hash(first) == hash(second))
