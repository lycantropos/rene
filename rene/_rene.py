from enum import (IntEnum,
                  unique)

MIN_CONTOUR_VERTICES_COUNT = 3
MIN_MULTISEGMENT_SEGMENTS_COUNT = 2


class Base(IntEnum):
    __module__ = 'rene'

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'.{self.name}')

    def __str__(self):
        return f'{type(self).__qualname__}.{self.name}'


@unique
class Relation(Base):
    """
    Represents kinds of relations in which geometries can be.
    Order of members assumes that conditions for previous ones do not hold.
    """
    #: intersection is empty
    DISJOINT = 0
    #: intersection is a strict subset of each of the geometries,
    #: has dimension less than at least of one of the geometries
    #: and if we traverse boundary of each of the geometries in any direction
    #: then boundary of the other geometry won't be on one of sides
    #: at each point of boundaries intersection
    TOUCH = 1
    #: intersection is a strict subset of each of the geometries,
    #: has dimension less than at least of one of the geometries
    #: and if we traverse boundary of each of the geometries in any direction
    #: then boundary of the other geometry will be on both sides
    #: at some point of boundaries intersection
    CROSS = 2
    #: intersection is a strict subset of each of the geometries
    #: and has the same dimension as geometries
    OVERLAP = 3
    #: interior of the geometry is a superset of the other
    COVER = 4
    #: boundary of the geometry contains
    #: at least one boundary point of the other, but not all,
    #: interior of the geometry contains other points of the other
    ENCLOSES = 5
    #: geometry is a strict superset of the other
    #: and interior/boundary of the geometry is a superset
    #: of interior/boundary of the other
    COMPOSITE = 6
    #: geometries are equal
    EQUAL = 7
    #: geometry is a strict subset of the other
    #: and interior/boundary of the geometry is a subset
    #: of interior/boundary of the other
    COMPONENT = 8
    #: at least one boundary point of the geometry
    #: lies on the boundary of the other, but not all,
    #: other points of the geometry lie in the interior of the other
    ENCLOSED = 9
    #: geometry is a subset of the interior of the other
    WITHIN = 10


@unique
class Orientation(Base):
    CLOCKWISE = -1
    COLLINEAR = 0
    COUNTERCLOCKWISE = 1
