from tests.exact_tests import strategies as _strategies

points = _strategies.points
segments_endpoints = _strategies.segments_endpoints
segments = _strategies.segments
relatable_compounds = (
    _strategies.contours | _strategies.multisegments | _strategies.segments
)
