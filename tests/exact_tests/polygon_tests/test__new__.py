from collections.abc import Sequence

from hypothesis import given

from rene.exact import Contour, Polygon
from tests.utils import are_sequences_equivalent

from . import strategies


@given(strategies.polygons_components)
def test_basic(components: tuple[Contour, Sequence[Contour]]) -> None:
    border, holes = components

    result = Polygon(border, holes)

    assert isinstance(result, Polygon)
    assert result.border == border
    assert are_sequences_equivalent(result.holes, holes)
