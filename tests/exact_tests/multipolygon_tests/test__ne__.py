from hypothesis import given

from rene.exact import Multipolygon
from tests.utils import equivalence

from . import strategies


@given(strategies.multipolygons)
def test_irreflexivity(multipolygon: Multipolygon) -> None:
    assert multipolygon == multipolygon


@given(strategies.multipolygons, strategies.multipolygons)
def test_symmetry(first: Multipolygon, second: Multipolygon) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.multipolygons, strategies.multipolygons)
def test_equivalents(first: Multipolygon, second: Multipolygon) -> None:
    assert equivalence(first != second, first != second)
