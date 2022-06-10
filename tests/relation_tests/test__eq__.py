from hypothesis import given

from rene import Relation
from tests.utils import (equivalence,
                         implication)
from . import strategies


@given(strategies.relations)
def test_reflexivity(relation: Relation) -> None:
    assert relation == relation


@given(strategies.relations, strategies.relations)
def test_symmetry(first: Relation, second: Relation) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.relations, strategies.relations,
       strategies.relations)
def test_transitivity(first: Relation,
                      second: Relation,
                      third: Relation) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.relations, strategies.relations)
def test_alternatives(first: Relation, second: Relation) -> None:
    assert equivalence(first == second, not first != second)
