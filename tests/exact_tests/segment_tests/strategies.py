from hypothesis import strategies

from rene.exact import Segment
from tests.strategies import points
from tests.utils import pack

segments_endpoints = strategies.lists(points,
                                      min_size=2,
                                      max_size=2,
                                      unique=True)
segments = segments_endpoints.map(pack(Segment))
