from typing import Sequence

from hypothesis import given

from rene.exact import Multisegment, Segment
from tests.utils import are_sequences_equivalent

from . import strategies


@given(strategies.multisegments_segments)
def test_basic(segments: Sequence[Segment]) -> None:
    result = Multisegment(segments)

    assert isinstance(result, Multisegment)
    assert are_sequences_equivalent(result.segments, segments)
