from collections.abc import Sequence

from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence, implication, reverse_sequence

from . import strategies


@given(strategies.multisegments_segments)
def test_reflexivity(segments: Sequence[Segment]) -> None:
    assert segments == segments


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_symmetry(first: Sequence[Segment], second: Sequence[Segment]) -> None:
    assert equivalence(first == second, second == first)


@given(
    strategies.multisegments_segments,
    strategies.multisegments_segments,
    strategies.multisegments_segments,
)
def test_transitivity(
    first: Sequence[Segment],
    second: Sequence[Segment],
    third: Sequence[Segment],
) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_alternatives(
    first: Sequence[Segment], second: Sequence[Segment]
) -> None:
    assert equivalence(first == second, first == second)


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_reversals(
    first: Sequence[Segment], second: Sequence[Segment]
) -> None:
    assert equivalence(
        first == second, reverse_sequence(first) == reverse_sequence(second)
    )
