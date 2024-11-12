from typing import Any

import pytest
from hypothesis import given

from rene import Location
from rene.exact import Polygon, Trapezoidation
from rene.hints import Seeder

from . import strategies


@given(strategies.polygons)
def test_basic_default_seeder(polygon: Polygon) -> None:
    result = Trapezoidation.from_polygon(polygon)

    assert isinstance(result, Trapezoidation)


@given(strategies.polygons, strategies.seeders)
def test_basic_custom_seeder(polygon: Polygon, seeder: Seeder) -> None:
    result = Trapezoidation.from_polygon(polygon, seeder=seeder)

    assert isinstance(result, Trapezoidation)


@given(strategies.polygons, strategies.seeders)
def test_contains(polygon: Polygon, seeder: Seeder) -> None:
    result = Trapezoidation.from_polygon(polygon, seeder=seeder)

    assert all(vertex in result for vertex in polygon.border.vertices)
    assert all(
        vertex in result for hole in polygon.holes for vertex in hole.vertices
    )


@given(strategies.polygons, strategies.seeders)
def test_locate(polygon: Polygon, seeder: Seeder) -> None:
    result = Trapezoidation.from_polygon(polygon, seeder=seeder)

    assert all(
        result.locate(vertex) is Location.BOUNDARY
        for vertex in polygon.border.vertices
    )
    assert all(
        result.locate(vertex) is Location.BOUNDARY
        for hole in polygon.holes
        for vertex in hole.vertices
    )


@given(strategies.polygons, strategies.invalid_seeds)
def test_invalid_seeders(polygon: Polygon, invalid_seed: Any) -> None:
    with pytest.raises((OverflowError, TypeError, ValueError)):
        Trapezoidation.from_polygon(polygon, seeder=lambda: invalid_seed)
