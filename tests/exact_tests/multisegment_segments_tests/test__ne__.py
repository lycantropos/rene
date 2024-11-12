from collections.abc import Sequence

from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence

from . import strategies


@given(strategies.multisegments_segments)
def test_irreflexivity(segments: Sequence[Segment]) -> None:
    assert segments == segments


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_symmetry(first: Sequence[Segment], second: Sequence[Segment]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_equivalents(
    first: Sequence[Segment], second: Sequence[Segment]
) -> None:
    assert equivalence(first != second, first != second)
