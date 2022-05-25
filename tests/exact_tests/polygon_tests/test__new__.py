from typing import (Sequence,
                    Tuple)

from hypothesis import given

from rene.exact import (Contour,
                        Polygon)
from . import strategies


@given(strategies.polygons_components)
def test_basic(components: Tuple[Contour, Sequence[Contour]]) -> None:
    border, holes = components

    result = Polygon(border, holes)

    assert isinstance(result, Polygon)
    assert result.border == border
    assert result.holes == holes
