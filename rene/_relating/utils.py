from __future__ import annotations

import typing as t
from itertools import chain

from rene import hints


def polygon_to_segments(
    polygon: hints.Polygon[hints.Scalar],
    bounding_box: hints.Box[hints.Scalar],
    /,
) -> t.Iterable[hints.Segment[hints.Scalar]]:
    return chain(
        polygon.border.segments,
        chain.from_iterable(
            hole.segments
            for hole in polygon.holes
            if not hole.bounding_box.disjoint_with(bounding_box)
        ),
    )
