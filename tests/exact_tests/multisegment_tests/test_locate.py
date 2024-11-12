from hypothesis import given

from rene.enums import Location
from rene.exact import Multisegment, Point
from tests.utils import (
    reverse_multisegment,
    reverse_multisegment_coordinates,
    reverse_point_coordinates,
)

from . import strategies


@given(strategies.multisegments, strategies.points)
def test_basic(multisegment: Multisegment, point: Point) -> None:
    result = multisegment.locate(point)

    assert isinstance(result, Location)


@given(strategies.multisegments)
def test_vertices(multisegment: Multisegment) -> None:
    assert [
        segment
        for segment in multisegment.segments
        if multisegment.locate(segment.start) is not Location.BOUNDARY
    ] == []
    assert [
        segment
        for segment in multisegment.segments
        if multisegment.locate(segment.end) is not Location.BOUNDARY
    ] == []


@given(strategies.multisegments, strategies.points)
def test_reversals(multisegment: Multisegment, point: Point) -> None:
    assert multisegment.locate(point) is reverse_multisegment(
        multisegment
    ).locate(point)
    assert multisegment.locate(point) is reverse_multisegment_coordinates(
        multisegment
    ).locate(reverse_point_coordinates(point))
