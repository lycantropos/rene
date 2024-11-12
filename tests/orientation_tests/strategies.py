from hypothesis import strategies

from rene import Orientation

orientations = strategies.sampled_from(
    [
        Orientation.CLOCKWISE,
        Orientation.COLLINEAR,
        Orientation.COUNTERCLOCKWISE,
    ]
)
