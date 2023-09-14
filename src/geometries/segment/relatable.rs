use crate::bounded::Bounded;
use std::hash::Hash;
use std::ops::Div;

use crate::geometries::{Contour, Empty, Multisegment, Point, Polygon};
use crate::operations::{CrossMultiply, IntersectCrossingSegments, Orient};
use crate::oriented::Oriented;
use crate::relatable::{Relatable, Relation};
use crate::relating::segment;
use crate::traits::{
    Contoural, Elemental, Multisegmental, MultisegmentalIndexSegment,
    Multivertexal, MultivertexalIndexVertex, Polygonal,
    PolygonalIntoIteratorHole, Segmental,
};

use super::types::Segment;

impl<Scalar> Relatable<&Empty> for &Segment<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar> Relatable for &Segment<Scalar>
where
    Point<Scalar>: PartialOrd,
    for<'a> &'a Point<Scalar>: Orient,
{
    fn equals_to(self, other: Self) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end)
            || self.start.eq(&other.end) && self.end.eq(&other.start)
    }

    fn relate_to(self, other: Self) -> Relation {
        segment::relate_to_segment(self, other)
    }
}

impl<Scalar> Relatable<&Contour<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + PartialOrd,
    for<'a> &'a Contour<Scalar>:
        Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        segment::relate_to_contour(self, other)
    }
}

impl<Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd>
    Relatable<&Multisegment<Scalar>> for &Segment<Scalar>
where
    for<'a> &'a Multisegment<Scalar>:
        Multisegmental<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Multisegment<Scalar>>:
        Segmental,
    Point<Scalar>: Eq + Hash + Ord,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a> &'a Point<Scalar>: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'a Scalar>
        + Orient,
{
    fn relate_to(self, other: &Multisegment<Scalar>) -> Relation {
        segment::relate_to_multisegment(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Polygon<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon<Scalar>> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<
            IndexSegment = Segment<Scalar>,
            IntoIteratorSegment = &'a Segment<Scalar>,
        > + Oriented,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>:
        Polygonal<Contour = &'a Contour<Scalar>, IndexHole = Contour<Scalar>>,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
{
    fn relate_to(self, other: &Polygon<Scalar>) -> Relation {
        segment::relate_to_polygon(self, other)
    }
}
