import typing as _t

from rene import exact as _exact

Compound = _t.Union[_exact.Empty, _exact.Multipolygon, _exact.Polygon]
