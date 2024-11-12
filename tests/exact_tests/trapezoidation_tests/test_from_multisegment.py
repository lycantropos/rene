from typing import Any

import pytest
from hypothesis import given

from rene import Location
from rene.exact import Multisegment, Trapezoidation
from rene.hints import Seeder

from . import strategies


@given(strategies.multisegments)
def test_basic_default_seeder(multisegment: Multisegment) -> None:
    result = Trapezoidation.from_multisegment(multisegment)

    assert isinstance(result, Trapezoidation)


@given(strategies.multisegments, strategies.seeders)
def test_basic_custom_seeder(
    multisegment: Multisegment, seeder: Seeder
) -> None:
    result = Trapezoidation.from_multisegment(multisegment, seeder=seeder)

    assert isinstance(result, Trapezoidation)


@given(strategies.multisegments, strategies.seeders)
def test_contains(multisegment: Multisegment, seeder: Seeder) -> None:
    result = Trapezoidation.from_multisegment(multisegment, seeder=seeder)

    assert all(segment.start in result for segment in multisegment.segments)
    assert all(segment.end in result for segment in multisegment.segments)


@given(strategies.multisegments, strategies.seeders)
def test_locate(multisegment: Multisegment, seeder: Seeder) -> None:
    result = Trapezoidation.from_multisegment(multisegment, seeder=seeder)

    assert all(
        result.locate(segment.start) is Location.BOUNDARY
        for segment in multisegment.segments
    )
    try:
        assert all(
            result.locate(segment.end) is Location.BOUNDARY
            for segment in multisegment.segments
        )
    except AssertionError:
        raise


@given(strategies.multisegments, strategies.invalid_seeds)
def test_invalid_seeders(
    multisegment: Multisegment, invalid_seed: Any
) -> None:
    with pytest.raises((OverflowError, TypeError, ValueError)):
        Trapezoidation.from_multisegment(
            multisegment, seeder=lambda: invalid_seed
        )
