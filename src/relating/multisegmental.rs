use std::hash::Hash;
use std::ops::{Div, Neg};

use traiter::numbers::Signed;

use crate::bounded::Bounded;
use crate::operations::{
    to_boxes_ids_with_intersection, CrossMultiply, DotMultiply,
    IntersectCrossingSegments, Orient, Square, SquaredMetric,
};
use crate::relatable::{Relatable, Relation};
use crate::relating::mixed;
use crate::sweeping::traits::{EventsQueue, SweepLine};
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, Multivertexal, MultivertexalIndexVertex,
    Polygonal, PolygonalHoles, PolygonalIntoIteratorHole, Segmental, Sequence,
};
use crate::{bounded, operations};

use super::event::Event;
use super::linear;
use super::segment;

pub(super) fn relate_to_multiregion<
    const REVERSE_ORIENTATION: bool,
    Multisegment,
    Border,
    Borders: Sequence<IndexItem = Border>,
    Point: Clone + Ord,
    Scalar: Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    multisegment: &Multisegment,
    borders: Borders,
) -> Relation
where
    mixed::Operation<true, REVERSE_ORIENTATION, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Border>: Segmental,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Multisegmental<IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    debug_assert!(borders.len() > 1);
    let multisegment_bounding_box = multisegment.to_bounding_box();
    let regions_bounding_boxes = borders
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let multiregion_bounding_box = {
        let (min_x, max_x, min_y, max_y) = operations::merge_bounds(
            regions_bounding_boxes.iter().map(|bounding_box| {
                (
                    *bounding_box.get_min_x(),
                    *bounding_box.get_max_x(),
                    *bounding_box.get_min_y(),
                    *bounding_box.get_max_y(),
                )
            }),
        );
        bounded::Box::new(min_x, max_x, min_y, max_y)
    };
    if multisegment_bounding_box.disjoint_with(&multiregion_bounding_box) {
        return Relation::Disjoint;
    }
    let multisegment_segments = multisegment.segments();
    let multisegment_bounding_boxes = multisegment_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let multisegment_intersecting_segments_ids =
        to_boxes_ids_with_intersection(
            &multisegment_bounding_boxes,
            &multiregion_bounding_box,
        );
    if multisegment_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    }
    let intersecting_borders_ids = to_boxes_ids_with_intersection(
        &regions_bounding_boxes,
        &multisegment_bounding_box,
    );
    if intersecting_borders_ids.is_empty() {
        return Relation::Disjoint;
    }
    let min_max_x = unsafe {
        multisegment_intersecting_segments_ids
            .iter()
            .map(|&index| multisegment_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    }
    .min(unsafe {
        intersecting_borders_ids
            .iter()
            .map(|&index| regions_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    });
    let max_min_x = unsafe {
        multisegment_intersecting_segments_ids
            .iter()
            .map(|&index| multisegment_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    }
    .max(unsafe {
        intersecting_borders_ids
            .iter()
            .map(|&index| regions_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    });
    let multisegment_intersecting_segments =
        multisegment_intersecting_segments_ids
            .into_iter()
            .filter_map(|index| {
                if max_min_x <= multisegment_bounding_boxes[index].get_max_x()
                    && multisegment_bounding_boxes[index].get_min_x()
                        <= min_max_x
                {
                    Some(&multisegment_segments[index])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    if multisegment_intersecting_segments.is_empty() {
        return Relation::Disjoint;
    }
    let intersecting_borders_segments = intersecting_borders_ids
        .iter()
        .map(|&border_id| borders[border_id].segments())
        .collect::<Vec<_>>();
    debug_assert!(!intersecting_borders_segments.is_empty());
    mixed::Operation::<true, REVERSE_ORIENTATION, Point>::from_segments_iterators(
        (
            multisegment_intersecting_segments.len(),
            multisegment_intersecting_segments.iter().copied().cloned(),
        ),
        (
            intersecting_borders_segments.iter().map(|border| border.len()).sum::<usize>(),
            intersecting_borders_segments.into_iter().flat_map(|border_segments| border_segments.into_iter().cloned())
        )
    )
    .into_relation(
        multisegment_intersecting_segments.len()
            == multisegment_segments.len(),
        min_max_x,
    )
}

pub(super) fn relate_to_multisegmental<
    const FIRST_IS_CONTOUR: bool,
    const SECOND_IS_CONTOUR: bool,
    First,
    Second,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment,
>(
    first: &First,
    second: &Second,
) -> Relation
where
    for<'a> &'a Output: Signed,
    for<'a> &'a First:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Second:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    let first_bounding_box = first.to_bounding_box();
    let second_bounding_box = second.to_bounding_box();
    if first_bounding_box.disjoint_with(&second_bounding_box) {
        return Relation::Disjoint;
    }
    let first_segments = first.segments();
    let first_bounding_boxes = first_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let first_intersecting_segments_ids = to_boxes_ids_with_intersection(
        &first_bounding_boxes,
        &second_bounding_box,
    );
    if first_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    }
    let second_segments = second.segments();
    if first_intersecting_segments_ids.len() == 1 {
        let first_intersecting_segment =
            &first_segments[first_intersecting_segments_ids[0]];
        return match if SECOND_IS_CONTOUR {
            segment::relate_to_contour_segments(
                first_intersecting_segment,
                second_segments.iter(),
            )
        } else {
            segment::relate_to_multisegment_segments(
                first_intersecting_segment,
                second_segments.iter(),
            )
        } {
            Relation::Component | Relation::Equal => Relation::Overlap,
            relation => relation,
        };
    }
    let second_bounding_boxes = second_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let second_intersecting_segments_ids = to_boxes_ids_with_intersection(
        &second_bounding_boxes,
        &first_bounding_box,
    );
    if second_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    } else if second_intersecting_segments_ids.len() == 1 {
        let second_intersecting_segment =
            &second_segments[second_intersecting_segments_ids[0]];
        return match if FIRST_IS_CONTOUR {
            segment::relate_to_contour_segments(
                second_intersecting_segment,
                first_intersecting_segments_ids
                    .iter()
                    .map(|&index| &first_segments[index]),
            )
        } else {
            segment::relate_to_multisegment_segments(
                second_intersecting_segment,
                first_intersecting_segments_ids
                    .iter()
                    .map(|&index| &first_segments[index]),
            )
        } {
            Relation::Component | Relation::Equal => Relation::Overlap,
            Relation::Composite
                if first_intersecting_segments_ids.len()
                    != first_segments.len() =>
            {
                Relation::Overlap
            }
            relation => relation.to_complement(),
        };
    }
    let min_max_x = unsafe {
        first_intersecting_segments_ids
            .iter()
            .map(|&index| first_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    }
    .min(unsafe {
        second_intersecting_segments_ids
            .iter()
            .map(|&index| second_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    });
    let max_min_x = unsafe {
        first_intersecting_segments_ids
            .iter()
            .map(|&index| first_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    }
    .max(unsafe {
        second_intersecting_segments_ids
            .iter()
            .map(|&index| second_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    });
    let first_intersecting_segments = first_intersecting_segments_ids
        .iter()
        .filter_map(|&index| {
            if max_min_x <= first_bounding_boxes[index].get_max_x()
                && first_bounding_boxes[index].get_min_x() <= min_max_x
            {
                Some(&first_segments[index])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if first_intersecting_segments.is_empty() {
        return Relation::Disjoint;
    }
    let second_intersecting_segments = second_intersecting_segments_ids
        .iter()
        .filter_map(|&index| {
            if max_min_x <= second_bounding_boxes[index].get_max_x()
                && second_bounding_boxes[index].get_min_x() <= min_max_x
            {
                Some(&second_segments[index])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    debug_assert!(!second_intersecting_segments.is_empty());
    linear::Operation::from((
        &first_intersecting_segments,
        &second_intersecting_segments,
    ))
    .into_relation(
        first_intersecting_segments.len() == first_segments.len(),
        second_intersecting_segments.len() == second_segments.len(),
        min_max_x,
    )
}

pub(super) fn relate_to_polygon<
    const IS_CONTOUR: bool,
    Border,
    Multisegment,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Polygon,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    multisegment: &Multisegment,
    polygon: &Polygon,
) -> Relation
where
    mixed::Operation<true, false, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    mixed::Operation<true, true, Point>:
        EventsQueue<Event = Event> + SweepLine<Event = Event>,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multisegmental>::IndexSegment: Segmental,
    for<'a, 'b> &'a <PolygonalIntoIteratorHole<&'b Polygon> as Multivertexal>::IndexVertex: Elemental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Border>: Elemental,
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Border: Bounded<&'a Scalar>
        + Contoural<IndexSegment = Segment, IntoIteratorSegment = &'a Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
    for<'a> &'a Polygon: Polygonal<Contour = &'a Border, IndexHole = Border>,
    for<'a> &'a Segment: Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
{
    let relation_without_holes = relate_to_region::<
        IS_CONTOUR,
        false,
        Multisegment,
        Border,
        Output,
        Point,
        Scalar,
        Segment,
    >(multisegment, polygon.border());
    if polygon.holes().len() > 0
        && matches!(
            relation_without_holes,
            Relation::Within | Relation::Enclosed
        )
    {
        let holes = polygon.holes();
        let relation_with_holes = if holes.len() == 1 {
            relate_to_region::<
                IS_CONTOUR,
                true,
                Multisegment,
                Border,
                Output,
                Point,
                Scalar,
                Segment,
            >(multisegment, &holes[0])
        } else {
            relate_to_multiregion::<
                true,
                Multisegment,
                Border,
                PolygonalHoles<&Polygon>,
                Point,
                Scalar,
                Segment,
            >(multisegment, polygon.holes())
        };
        match relation_with_holes {
            Relation::Disjoint => relation_without_holes,
            Relation::Touch => Relation::Enclosed,
            Relation::Enclosed => Relation::Touch,
            Relation::Within => Relation::Disjoint,
            _ => relation_with_holes,
        }
    } else {
        relation_without_holes
    }
}

pub(super) fn relate_to_region<
    const IS_CONTOUR: bool,
    const REVERSE_ORIENTATION: bool,
    Multisegment,
    Border,
    Output: Div<Output = Output>
        + Neg<Output = Output>
        + Ord
        + Square<Output = Output>,
    Point: Clone + Hash + Ord,
    Scalar: Div<Output = Scalar> + Hash + Ord,
    Segment: Clone + Segmental<Endpoint = Point>,
>(
    multisegmental: &Multisegment,
    border: &Border,
) -> Relation
where
    for<'a, 'b> &'a bounded::Box<&'b Scalar>: Relatable,
    for<'a, 'b> &'a Segment:
        Bounded<&'a Scalar> + Segmental<Endpoint = &'a Point>,
    for<'a, 'b> linear::Operation<Point>: From<(&'a [&'b Segment], &'a [&'b Segment])>
        + EventsQueue<Event = Event>
        + SweepLine<Event = Event>,
    for<'a> &'a Border:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Multisegment:
        Bounded<&'a Scalar> + Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Output: Signed,
    for<'a> &'a Point: CrossMultiply<Output = Scalar>
        + DotMultiply<Output = Output>
        + Elemental<Coordinate = &'a Scalar>
        + IntersectCrossingSegments<Output = Point>
        + Orient
        + SquaredMetric<Output = Output>,
{
    let multisegmental_bounding_box = multisegmental.to_bounding_box();
    let region_bounding_box = border.to_bounding_box();
    if multisegmental_bounding_box.disjoint_with(&region_bounding_box) {
        return Relation::Disjoint;
    }
    let multisegmental_segments = multisegmental.segments();
    let multisegmental_bounding_boxes = multisegmental_segments
        .iter()
        .map(Bounded::to_bounding_box)
        .collect::<Vec<_>>();
    let multisegmental_intersecting_segments_ids =
        to_boxes_ids_with_intersection(
            &multisegmental_bounding_boxes,
            &region_bounding_box,
        );
    if multisegmental_intersecting_segments_ids.is_empty() {
        return Relation::Disjoint;
    }
    let region_segments = border.segments();
    if multisegmental_intersecting_segments_ids.len() == 1 {
        let multisegmental_intersecting_segment = &multisegmental_segments
            [multisegmental_intersecting_segments_ids[0]];
        return match segment::relate_to_region::<
            REVERSE_ORIENTATION,
            Border,
            Point,
            Scalar,
            Segment,
        >(multisegmental_intersecting_segment, border)
        {
            Relation::Component => Relation::Touch,
            Relation::Enclosed | Relation::Within => Relation::Cross,
            relation => relation,
        };
    }
    let min_max_x = unsafe {
        multisegmental_intersecting_segments_ids
            .iter()
            .map(|&index| multisegmental_bounding_boxes[index].get_max_x())
            .max()
            .unwrap_unchecked()
    }
    .min(region_bounding_box.get_max_x());
    let max_min_x = unsafe {
        multisegmental_intersecting_segments_ids
            .iter()
            .map(|&index| multisegmental_bounding_boxes[index].get_min_x())
            .min()
            .unwrap_unchecked()
    }
    .max(region_bounding_box.get_min_x());
    let multisegmental_intersecting_segments =
        multisegmental_intersecting_segments_ids
            .iter()
            .filter_map(|&index| {
                if max_min_x
                    <= multisegmental_bounding_boxes[index].get_max_x()
                    && multisegmental_bounding_boxes[index].get_min_x()
                        <= min_max_x
                {
                    Some(&multisegmental_segments[index])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    if multisegmental_intersecting_segments.is_empty() {
        return Relation::Disjoint;
    }
    mixed::Operation::<true, REVERSE_ORIENTATION, Point>::from_segments_iterators(
        (
            multisegmental_intersecting_segments.len(),
            multisegmental_intersecting_segments.iter().copied().cloned(),
        ),
        (region_segments.len(), region_segments.iter().cloned()),
    )
    .into_relation(
        multisegmental_intersecting_segments.len()
            == multisegmental_segments.len(),
        min_max_x,
    )
}
