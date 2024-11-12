from hypothesis import given

from rene import Relation

from . import strategies


@given(strategies.relations)
def test_round_trip(relation: Relation) -> None:
    result = repr(relation)

    assert eval(result, {Relation.__qualname__: Relation}) is relation
