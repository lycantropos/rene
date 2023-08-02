import typing as t

from hypothesis import given

from rene.exact import Segment
from tests.utils import equivalence
from . import strategies


@given(strategies.multisegments_segments)
def test_irreflexivity(segments: t.Sequence[Segment]) -> None:
    assert not segments != segments


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_symmetry(first: t.Sequence[Segment],
                  second: t.Sequence[Segment]) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multisegments_segments, strategies.multisegments_segments)
def test_equivalents(first: t.Sequence[Segment],
                     second: t.Sequence[Segment]) -> None:
    assert equivalence(first != second, not first == second)
