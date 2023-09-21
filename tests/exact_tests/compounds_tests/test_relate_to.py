from hypothesis import given

from rene import Relation
from tests.exact_tests.hints import (Compound,
                                     MaybeLinearCompound)
from tests.utils import reverse_compound_coordinates
from . import strategies


@given(strategies.maybe_linear_compounds, strategies.compounds)
def test_basic(first: MaybeLinearCompound, second: Compound) -> None:
    result = first.relate_to(second)

    assert isinstance(result, Relation)


@given(strategies.maybe_linear_compounds, strategies.maybe_linear_compounds)
def test_complement(first: MaybeLinearCompound,
                    second: MaybeLinearCompound) -> None:
    assert first.relate_to(second) is second.relate_to(first).complement


@given(strategies.maybe_linear_compounds, strategies.compounds)
def test_reversals(first: MaybeLinearCompound, second: Compound) -> None:
    assert (first.relate_to(second)
            is reverse_compound_coordinates(first).relate_to(
                    reverse_compound_coordinates(second)
            ))
