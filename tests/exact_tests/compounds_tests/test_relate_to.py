from hypothesis import given

from rene import Relation
from tests.exact_tests.hints import Compound
from tests.utils import reverse_compound_coordinates

from . import strategies


@given(strategies.compounds, strategies.compounds)
def test_basic(first: Compound, second: Compound) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.compounds, strategies.compounds)
def test_complement(first: Compound, second: Compound) -> None:
    assert first.relate_to(second) is second.relate_to(first).complement


@given(strategies.compounds, strategies.compounds)
def test_reversals(first: Compound, second: Compound) -> None:
    assert first.relate_to(second) is reverse_compound_coordinates(
        first
    ).relate_to(reverse_compound_coordinates(second))
