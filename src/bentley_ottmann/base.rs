use core::convert::From;

use crate::constants::MIN_MULTISEGMENT_SEGMENTS_COUNT;
use crate::contracts::are_contour_vertices_non_degenerate;
use crate::operations::Orient;
use crate::relatable::Relation;
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental, Multivertexal,
    Segmental,
};

use super::sweep::{Intersection, Sweep};

pub(crate) fn is_contour_valid<Contour, Point: Ord, Scalar, Segment>(
    contour: &Contour,
) -> bool
where
    Sweep<Point>: Iterator<Item = Intersection<Point>>
        + for<'a, 'b> From<&'a <&'b Contour as Multisegmental>::Segments>,
    for<'a> &'a Contour:
        Contoural<IndexVertex = Point, IndexSegment = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar> + Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    are_contour_vertices_non_degenerate(
        &contour.vertices().iter().collect::<Vec<_>>(),
    ) && {
        let segments = contour.segments();
        segments.iter().all(|segment| {
            let (start, end) = segment.endpoints();
            start != end
        }) && {
            let mut sweep = Sweep::from(&segments);
            let mut neighbour_segments_touches_count = 0usize;
            while let Some(intersection) = sweep.next() {
                debug_assert_eq!(
                    intersection.start == intersection.end,
                    matches!(
                        intersection.relation,
                        Relation::Touch | Relation::Cross
                    )
                );
                let touches_at_vertices =
                    intersection.relation == Relation::Touch
                        && (intersection.start.eq(sweep.get_segment_start(
                            intersection.first_segment_id,
                        )) || intersection.start.eq(sweep
                            .get_segment_end(intersection.first_segment_id)))
                        && (intersection.start.eq(sweep.get_segment_start(
                            intersection.second_segment_id,
                        )) || intersection.start.eq(sweep
                            .get_segment_end(intersection.second_segment_id)));
                let neighbour_segments_intersection = intersection
                    .first_segment_id
                    .abs_diff(intersection.second_segment_id)
                    == 1
                    || (intersection.first_segment_id == segments.len() - 1
                        && intersection.second_segment_id == 0)
                    || (intersection.second_segment_id == segments.len() - 1
                        && intersection.first_segment_id == 0);
                if !(touches_at_vertices && neighbour_segments_intersection) {
                    return false;
                }
                neighbour_segments_touches_count += 1;
            }
            neighbour_segments_touches_count == segments.len()
        }
    }
}

pub(crate) fn is_multisegment_valid<
    'a,
    Multisegment,
    Point: PartialEq,
    Segment,
>(
    multisegment: &'a Multisegment,
) -> bool
where
    Sweep<Point>: for<'b, 'c> From<&'b <&'c Multisegment as Multisegmental>::Segments>
        + Iterator<Item = Intersection<Point>>,
    for<'b> &'b Multisegment: Multisegmental<IndexSegment = Segment>,
    for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
{
    let segments = multisegment.segments();
    segments.len() >= MIN_MULTISEGMENT_SEGMENTS_COUNT
        && segments.iter().all(|segment| {
            let (start, end) = segment.endpoints();
            start != end
        })
        && Sweep::from(&segments)
            .all(|intersection| intersection.relation == Relation::Touch)
}
