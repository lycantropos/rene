from hypothesis import strategies

from rene import Orientation

orientations_values = strategies.sampled_from([-1, 0, 1])
orientations = strategies.sampled_from([
    Orientation.CLOCKWISE, Orientation.COLLINEAR, Orientation.COUNTERCLOCKWISE
])
