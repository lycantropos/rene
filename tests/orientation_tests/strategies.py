from hypothesis import strategies

from rene.enums import Orientation

orientations = strategies.sampled_from(
    [
        Orientation.CLOCKWISE,
        Orientation.COLLINEAR,
        Orientation.COUNTERCLOCKWISE,
    ]
)
