from __future__ import annotations

import typing as t
from itertools import chain

if t.TYPE_CHECKING:
    from rene import hints


def polygon_to_segments(
    polygon: hints.Polygon[hints.ScalarT],
    bounding_box: hints.Box[hints.ScalarT],
    /,
) -> t.Iterable[hints.Segment[hints.ScalarT]]:
    return chain(
        polygon.border.segments,
        chain.from_iterable(
            hole.segments
            for hole in polygon.holes
            if not hole.bounding_box.disjoint_with(bounding_box)
        ),
    )
