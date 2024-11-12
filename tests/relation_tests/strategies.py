from hypothesis import strategies

from rene.enums import Relation

relations = strategies.sampled_from(
    [
        Relation.COMPONENT,
        Relation.COMPOSITE,
        Relation.COVER,
        Relation.CROSS,
        Relation.DISJOINT,
        Relation.ENCLOSED,
        Relation.ENCLOSES,
        Relation.EQUAL,
        Relation.OVERLAP,
        Relation.TOUCH,
        Relation.WITHIN,
    ]
)
