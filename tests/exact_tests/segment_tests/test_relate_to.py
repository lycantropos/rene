from hypothesis import given

from rene import Relation
from rene.exact import Segment
from tests.exact_tests.hints import Compound
from tests.utils import (reverse_compound_coordinates,
                         reverse_segment_coordinates,
                         reverse_segment_endpoints)
from . import strategies


@given(strategies.segments, strategies.relatable_compounds)
def test_basic(first: Segment, second: Compound) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.segments, strategies.relatable_compounds)
def test_reversals(first: Segment, second: Compound) -> None:
    assert (first.relate_to(second)
            is reverse_segment_endpoints(first).relate_to(second))
    assert (first.relate_to(second)
            is reverse_segment_coordinates(first).relate_to(
                    reverse_compound_coordinates(second)
            ))
