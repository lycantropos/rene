use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::operations::{
    CrossMultiply, DotMultiply, IntersectCrossingSegments, Orient, Square,
    SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Multisegmental, MultisegmentalIndexSegment,
    Segmental,
};

use super::event::Event;
use super::linear::Operation;
use super::multisegmental::relate_to_multisegmental;
use super::segment::relate_to_contour as relate_segment_to_contour;

pub(crate) fn relate_to_contour<
    Contour,
    Point: Clone + Hash + Ord,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    first: &Contour,
    second: &Contour,
) -> Relation
where
    for<'a> &'a Output: Signed,
    for<'a> &'a Contour:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    relate_to_multisegmental::<
        true,
        true,
        Contour,
        Contour,
        Point,
        Output,
        Scalar,
        Segment,
    >(first, second)
}

pub(crate) fn relate_to_multisegment<
    Contour,
    Multisegment,
    Point: Clone + Hash + Ord,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    contour: &Contour,
    multisegment: &Multisegment,
) -> Relation
where
    for<'a> &'a Output: Signed,
    for<'a> &'a Contour:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    relate_to_multisegmental::<
        true,
        false,
        Contour,
        Multisegment,
        Point,
        Output,
        Scalar,
        Segment,
    >(contour, multisegment)
}

pub(crate) fn relate_to_segment<Contour, Point: Clone + PartialOrd, Segment>(
    contour: &Contour,
    segment: &Segment,
) -> Relation
where
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Contour>: Segmental,
    for<'a> &'a Contour: Contoural<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Point: Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    relate_segment_to_contour(segment, contour).to_complement()
}
