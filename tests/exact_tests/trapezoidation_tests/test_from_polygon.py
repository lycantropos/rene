from typing import Any

import pytest
from hypothesis import given

from rene._exact import (Polygon,
                         Trapezoidation)
from rene.hints import Seeder
from . import strategies


@given(strategies.polygons)
def test_basic_default_seeder(polygon: Polygon) -> None:
    result = Trapezoidation.from_polygon(polygon)

    assert isinstance(result, Trapezoidation)


@given(strategies.polygons, strategies.seeders)
def test_basic_custom_seeder(polygon: Polygon, seeder: Seeder) -> None:
    result = Trapezoidation.from_polygon(polygon,
                                         seeder=seeder)

    assert isinstance(result, Trapezoidation)


@given(strategies.polygons, strategies.invalid_seeds)
def test_invalid_seeders(polygon: Polygon,
                         invalid_seed: Any) -> None:
    with pytest.raises((ValueError, TypeError)):
        Trapezoidation.from_polygon(polygon,
                                    seeder=lambda: invalid_seed)
