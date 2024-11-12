from hypothesis import given

from rene import Relation
from rene.exact import Box
from tests.utils import equivalence

from . import strategies


@given(strategies.boxes, strategies.boxes)
def test_basic(first: Box, second: Box) -> None:
    result = first.enclosed_by(second)

    assert isinstance(result, bool)


@given(strategies.boxes, strategies.boxes)
def test_alternatives(first: Box, second: Box) -> None:
    assert equivalence(
        first.enclosed_by(second), first.relate_to(second) is Relation.ENCLOSED
    )
