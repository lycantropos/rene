from hypothesis import given

from rene.exact import (Box,
                        Multisegment)
from tests.utils import (is_multisegment_inside_box,
                         reverse_box_coordinates,
                         reverse_multisegment_coordinates)
from . import strategies


@given(strategies.multisegments)
def test_basic(multisegment: Multisegment) -> None:
    result = multisegment.bounding_box

    assert isinstance(result, Box)


@given(strategies.multisegments)
def test_relations(multisegment: Multisegment) -> None:
    result = multisegment.bounding_box

    assert is_multisegment_inside_box(multisegment, result)


@given(strategies.multisegments)
def test_reversals(multisegment: Multisegment) -> None:
    assert (reverse_box_coordinates(multisegment.bounding_box)
            == reverse_multisegment_coordinates(multisegment).bounding_box)
