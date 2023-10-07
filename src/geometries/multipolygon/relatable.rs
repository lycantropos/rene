use crate::bounded::Bounded;
use crate::geometries::{
    Contour, Empty, Multisegment, Point, Polygon, Segment,
};
use crate::operations::{IntersectCrossingSegments, Orient};
use crate::relatable::{Relatable, Relation};
use crate::relating::{mixed, multipolygon, shaped, Event};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multipolygonal, MultipolygonalIntoIteratorPolygon,
    Multisegmental, MultisegmentalIndexSegment, MultivertexalIndexVertex,
    Polygonal, PolygonalContour, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental,
};

use super::types::Multipolygon;

impl<Scalar> Relatable<&Empty> for &Multipolygon<Scalar> {
    fn relate_to(self, _other: &Empty) -> Relation {
        Relation::Disjoint
    }
}

impl<Scalar: Ord> Relatable<&Contour<Scalar>> for &Multipolygon<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    mixed::Operation<true, Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<
            IndexSegment = Segment<Scalar>,
            IntoIteratorSegment = &'a Segment<Scalar>,
        >,
    for<'a> &'a Multipolygon<Scalar>:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Contour<Scalar>) -> Relation {
        multipolygon::relate_to_contour(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Multipolygon<Scalar>> for &Multipolygon<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    shaped::Operation<Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Multipolygon<Scalar>:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Multipolygon<Scalar>) -> Relation {
        multipolygon::relate_to_multipolygon(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Multisegment<Scalar>> for &Multipolygon<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    mixed::Operation<true, Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Multipolygon<Scalar>:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Multisegment<Scalar>:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Multisegment<Scalar>) -> Relation {
        multipolygon::relate_to_multisegment(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Polygon<Scalar>> for &Multipolygon<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    shaped::Operation<Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Multipolygon<Scalar>:
        Bounded<&'a Scalar> + Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Polygon<Scalar>) -> Relation {
        multipolygon::relate_to_polygon(self, other)
    }
}

impl<Scalar: Ord> Relatable<&Segment<Scalar>> for &Multipolygon<Scalar>
where
    mixed::Operation<true, Point<Scalar>>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone + Segmental<Endpoint = Point<Scalar>>,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>
        + Contoural<IntoIteratorSegment = &'a Segment<Scalar>>,
    for<'a> &'a Multipolygon<Scalar>:
        Multipolygonal<IndexPolygon = Polygon<Scalar>>,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point<Scalar>>
        + Orient,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>
        + Polygonal<
            Contour = &'a Contour<Scalar>,
            IntoIteratorHole = &'a Contour<Scalar>,
        >,
    for<'a> &'a Segment<Scalar>:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point<Scalar>>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalContour<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<
            MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour<Scalar>>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a, 'b> &'a PolygonalIndexHole<
        MultipolygonalIntoIteratorPolygon<&'b Multipolygon<Scalar>>,
    >: Contoural,
    for<'a, 'b> &'a PolygonalIndexHole<&'b Polygon<Scalar>>: Contoural,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<
        &'b PolygonalIndexHole<
            MultipolygonalIntoIteratorPolygon<&'c Multipolygon<Scalar>>,
        >,
    >: Elemental,
    for<'a, 'b, 'c> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<&'c Polygon<Scalar>>,
    >: Segmental,
    for<'a, 'b, 'c> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<&'c Polygon<Scalar>>>:
        Elemental,
{
    fn relate_to(self, other: &Segment<Scalar>) -> Relation {
        multipolygon::relate_to_segment(self, other)
    }
}
