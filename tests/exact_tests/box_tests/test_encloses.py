from hypothesis import given

from rene.enums import Relation
from rene.exact import Box
from tests.utils import equivalence

from . import strategies


@given(strategies.boxes, strategies.boxes)
def test_basic(first: Box, second: Box) -> None:
    result = first.encloses(second)

    assert isinstance(result, bool)


@given(strategies.boxes, strategies.boxes)
def test_alternatives(first: Box, second: Box) -> None:
    assert equivalence(
        first.encloses(second), first.relate_to(second) is Relation.ENCLOSES
    )
