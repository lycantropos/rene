from typing import Sequence

from hypothesis import given

from rene.exact import (Multisegment,
                        Segment)
from . import strategies


@given(strategies.multisegments_segments)
def test_basic(segments: Sequence[Segment]) -> None:
    result = Multisegment(segments)

    assert isinstance(result, Multisegment)
    assert result.segments == segments
