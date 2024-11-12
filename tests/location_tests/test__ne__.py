from hypothesis import given

from rene.enums import Location
from tests.utils import equivalence

from . import strategies


@given(strategies.locations)
def test_irreflexivity(location: Location) -> None:
    assert location == location


@given(strategies.locations, strategies.locations)
def test_symmetry(first: Location, second: Location) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.locations, strategies.locations)
def test_equivalents(first: Location, second: Location) -> None:
    assert equivalence(first != second, first != second)
