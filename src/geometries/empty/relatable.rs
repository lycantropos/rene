use crate::geometries::{Contour, Multipolygon, Multisegment, Polygon, Segment};
use crate::relatable::{Relatable, Relation};

use super::types::Empty;

impl Relatable for &Empty {
    fn relate_to(self, _other: Self) -> Relation {
        Relation::Equal
    }
}

impl<Scalar> Relatable<&Contour<Scalar>> for &Empty {
    fn relate_to(self, _other: &Contour<Scalar>) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable<&Multipolygon<Scalar>> for &Empty {
    fn relate_to(self, _other: &Multipolygon<Scalar>) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable<&Multisegment<Scalar>> for &Empty {
    fn relate_to(self, _other: &Multisegment<Scalar>) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable<&Polygon<Scalar>> for &Empty {
    fn relate_to(self, _other: &Polygon<Scalar>) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable<&Segment<Scalar>> for &Empty {
    fn relate_to(self, _other: &Segment<Scalar>) -> Relation {
        Relation::Disjoint
    }
}
