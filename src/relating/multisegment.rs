use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded;
use crate::bounded::Bounded;
use crate::operations::{
    to_boxes_ids_with_intersection, CrossMultiply, DotMultiply,
    IntersectCrossingSegments, Orient, Square, SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, Segmental,
};

use super::event::{is_left_event, Event};
use super::linear::Operation;
use super::multisegmental::relate_to_multisegmental;
use super::segment::relate_to_multisegment as relate_segment_to_multisegment;

pub(crate) fn relate_to_contour<
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
    multisegment: &Multisegment,
    contour: &Contour,
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
        false,
        true,
        Multisegment,
        Contour,
        Point,
        Output,
        Scalar,
        Segment,
    >(multisegment, contour)
}

pub(crate) fn relate_to_multisegment<
    Multisegment,
    Point: Clone + Hash + Ord,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    first: &Multisegment,
    second: &Multisegment,
) -> Relation
where
    for<'a> &'a Output: Signed,
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
        false,
        false,
        Multisegment,
        Multisegment,
        Point,
        Output,
        Scalar,
        Segment,
    >(first, second)
}

pub(crate) fn relate_to_segment<
    'a,
    Multisegment,
    Point: Hash + Ord,
    Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd,
    Segment: 'a,
>(
    multisegment: &'a Multisegment,
    mut start: &'a Point,
    mut end: &'a Point,
) -> Relation
where
    &'a Multisegment: Multisegmental<IntoIteratorSegment = &'a Segment>,
    &'a Segment: Segmental<Endpoint = &'a Point>,
    for<'b> &'b MultisegmentalIndexSegment<&'a Multisegment>: Segmental,
    for<'b> &'b Point: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    relate_segment_to_multisegment(start, end, multisegment).to_complement()
}
