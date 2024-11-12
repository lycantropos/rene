from hypothesis import given

from rene.enums import Relation

from . import strategies


@given(strategies.relations)
def test_basic(relation: Relation) -> None:
    result = relation.complement

    assert isinstance(result, Relation)


@given(strategies.relations)
def test_involution(relation: Relation) -> None:
    assert relation.complement.complement is relation
