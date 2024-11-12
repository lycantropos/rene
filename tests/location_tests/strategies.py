from hypothesis import strategies

from rene.enums import Location

locations = strategies.sampled_from(
    [Location.BOUNDARY, Location.EXTERIOR, Location.INTERIOR]
)
