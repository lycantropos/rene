from hypothesis import given

from rene import Relation
from tests.utils import implication

from . import strategies


@given(strategies.relations)
def test_determinism(relation: Relation) -> None:
    result = hash(relation)

    assert result == hash(relation)


@given(strategies.relations, strategies.relations)
def test_preserving_equality(first: Relation, second: Relation) -> None:
    assert implication(first == second, hash(first) == hash(second))
