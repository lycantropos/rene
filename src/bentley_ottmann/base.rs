use core::convert::From;

use crate::constants::MIN_MULTISEGMENT_SEGMENTS_COUNT;
use crate::contracts::are_contour_vertices_non_degenerate;
use crate::operations::Orient;
use crate::relatable::Relation;
use crate::traits::{Contoural, Elemental, Multisegmental, Multivertexal, Segmental};

use super::event::{is_left_event, Event};
use super::events_registry::EventsRegistry;
use super::sweep::{Intersection, Sweep};

pub(crate) fn is_contour_valid<'a, Contour, Point: Ord, Scalar, Segment: 'a>(
    contour: &'a Contour,
) -> bool
where
    &'a Contour: Contoural<Vertex = Point, Segment = Segment>,
    Segment: Segmental<Endpoint = Point>,
    Sweep<Point>: From<&'a Contour> + Iterator<Item = Intersection<Point>>,
    for<'b> &'b Point: Elemental<Coordinate = &'b Scalar> + Orient,
{
    are_contour_vertices_non_degenerate(&contour.vertices().collect::<Vec<_>>()) && {
        contour.segments().all(|segment| {
            let (start, end) = segment.endpoints();
            start != end
        }) && {
            let mut sweep = Sweep::from(contour);
            let mut neighbour_segments_touches_count = 0usize;
            while let Some(intersection) = sweep.next() {
                debug_assert_eq!(
                    intersection.start == intersection.end,
                    matches!(intersection.relation, Relation::Touch | Relation::Cross)
                );
                let touches_at_vertices = intersection.relation == Relation::Touch
                    && (intersection
                        .start
                        .eq(sweep.get_segment_start(intersection.first_segment_id))
                        || intersection
                            .start
                            .eq(sweep.get_segment_end(intersection.first_segment_id)))
                    && (intersection
                        .start
                        .eq(sweep.get_segment_start(intersection.second_segment_id))
                        || intersection
                            .start
                            .eq(sweep.get_segment_end(intersection.second_segment_id)));
                let neighbour_segments_intersection = intersection
                    .first_segment_id
                    .abs_diff(intersection.second_segment_id)
                    == 1
                    || (intersection.first_segment_id == contour.segments_count() - 1
                        && intersection.second_segment_id == 0)
                    || (intersection.second_segment_id == contour.segments_count() - 1
                        && intersection.first_segment_id == 0);
                if !(touches_at_vertices && neighbour_segments_intersection) {
                    return false;
                }
                neighbour_segments_touches_count += 1;
            }
            neighbour_segments_touches_count == contour.segments_count()
        }
    }
}

pub(crate) fn is_multisegment_valid<'a, Multisegment, Point: PartialEq, Segment>(
    multisegment: &'a Multisegment,
) -> bool
where
    &'a Multisegment: Multisegmental<Segment = Segment>,
    Segment: Segmental<Endpoint = Point>,
    Sweep<Point>: From<&'a Multisegment> + Iterator<Item = Intersection<Point>>,
{
    multisegment.segments_count() >= MIN_MULTISEGMENT_SEGMENTS_COUNT
        && multisegment.segments().all(|segment| {
            let (start, end) = segment.endpoints();
            start != end
        })
        && Sweep::from(multisegment).all(|intersection| intersection.relation == Relation::Touch)
}

pub(crate) fn to_unique_non_crossing_or_overlapping_segments<
    Point: Clone,
    Segment: From<(Point, Point)>,
>(
    segments: &[Segment],
) -> Vec<Segment>
where
    Segment: Segmental<Endpoint = Point>,
    for<'a> EventsRegistry<Point, true>: From<&'a [Segment]> + Iterator<Item = Event>,
{
    let mut result = Vec::with_capacity(segments.len());
    let mut events_registry = EventsRegistry::<Point, true>::from(segments);
    while let Some(event) = events_registry.next() {
        if !is_left_event(event) {
            result.push(Segment::from((
                events_registry.get_event_start(event).clone(),
                events_registry.get_event_end(event).clone(),
            )));
        }
    }
    result
}
