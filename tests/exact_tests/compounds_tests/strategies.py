from typing import Any

from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies
from tests.utils import to_pairs, to_triplets

empty_geometries = _strategies.empty_geometries
compounds = (
    empty_geometries
    | _strategies.segments
    | _strategies.multisegments
    | _strategies.contours
    | _strategies.polygons
    | _strategies.multipolygons
)
idempotent_linear_compounds = _strategies.segments | _strategies.multisegments
linear_compounds = idempotent_linear_compounds | _strategies.contours
shaped_compounds = _strategies.polygons | _strategies.multipolygons
maybe_shaped_compounds = empty_geometries | shaped_compounds
maybe_linear_compounds = empty_geometries | linear_compounds
idempotent_maybe_linear_compounds = (
    empty_geometries | idempotent_linear_compounds
)
_closed_compounds_strategies: _st.SearchStrategy[_st.SearchStrategy[Any]] = (
    _st.sampled_from([maybe_linear_compounds, maybe_shaped_compounds])
)
_closed_idempotent_compounds_strategies: _st.SearchStrategy[
    _st.SearchStrategy[Any]
] = _st.sampled_from(
    [idempotent_maybe_linear_compounds, maybe_shaped_compounds]
)
closed_compounds_pairs = _closed_compounds_strategies.flatmap(to_pairs)
closed_compounds_triplets = _closed_compounds_strategies.flatmap(to_triplets)
closed_idempotent_compounds = (
    idempotent_maybe_linear_compounds | maybe_shaped_compounds
)
closed_idempotent_compounds_pairs = (
    _closed_idempotent_compounds_strategies.flatmap(to_pairs)
)
closed_idempotent_compounds_triplets = (
    _closed_idempotent_compounds_strategies.flatmap(to_triplets)
)
points = _strategies.points
