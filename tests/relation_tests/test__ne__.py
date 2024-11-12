from hypothesis import given

from rene.enums import Relation
from tests.utils import equivalence

from . import strategies


@given(strategies.relations)
def test_irreflexivity(relation: Relation) -> None:
    assert relation == relation


@given(strategies.relations, strategies.relations)
def test_symmetry(first: Relation, second: Relation) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.relations, strategies.relations)
def test_equivalents(first: Relation, second: Relation) -> None:
    assert equivalence(first != second, first != second)
