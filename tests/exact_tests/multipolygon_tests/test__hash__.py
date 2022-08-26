from hypothesis import given

from rene.exact import Multipolygon
from tests.utils import implication
from . import strategies


@given(strategies.multipolygons)
def test_determinism(multipolygon: Multipolygon) -> None:
    result = hash(multipolygon)

    assert result == hash(multipolygon)


@given(strategies.multipolygons, strategies.multipolygons)
def test_preserving_equality(first: Multipolygon,
                             second: Multipolygon) -> None:
    assert implication(first == second, hash(first) == hash(second))
