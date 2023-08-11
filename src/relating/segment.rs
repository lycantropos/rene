use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Div;

use crate::operations::{
    is_point_in_segment, point_vertex_line_divides_angle,
    subtract_segments_overlap, to_segments_intersection_scale, to_sorted_pair,
    CrossMultiply, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, Segmental,
};

pub(crate) fn relate_to_contour<
    'a,
    Contour,
    Point: Clone + PartialOrd,
    Segment,
>(
    start: &Point,
    end: &Point,
    contour: &'a Contour,
) -> Relation
where
    for<'b> &'b Contour: Contoural<IndexSegment = Segment>,
    for<'b> &'b Point: Orient,
    for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
{
    let mut has_no_cross = true;
    let mut has_no_touch = true;
    let mut last_touched_edge_index: Option<usize> = None;
    let mut last_touched_edge_start: Option<Point> = None;
    let contour_segments = contour.segments();
    for (index, contour_segment) in contour_segments.iter().enumerate() {
        let (contour_segment_start, contour_segment_end) =
            contour_segment.endpoints();
        let relation = relate_to_segment(
            start,
            end,
            contour_segment_start,
            contour_segment_end,
        );
        match relation {
            Relation::Component | Relation::Equal => {
                return Relation::Component
            }
            Relation::Composite | Relation::Overlap => {
                return Relation::Overlap
            }
            Relation::Cross => {
                if has_no_cross {
                    has_no_cross = false;
                }
            }
            Relation::Touch => {
                if has_no_touch {
                    has_no_touch = false;
                } else if has_no_cross {
                    debug_assert!(last_touched_edge_index.is_some());
                    debug_assert!(last_touched_edge_start.is_some());
                    if index
                        - unsafe { last_touched_edge_index.unwrap_unchecked() }
                        == 1
                        && contour_segment_start.ne(start)
                        && contour_segment_start.ne(end)
                        && contour_segment_end.ne(start)
                        && contour_segment_end.ne(end)
                        && start.orient(end, contour_segment_start)
                            == Orientation::Collinear
                        && point_vertex_line_divides_angle(
                            start,
                            contour_segment_start,
                            contour_segment_end,
                            unsafe {
                                &last_touched_edge_start.unwrap_unchecked()
                            },
                        )
                    {
                        has_no_cross = false;
                    }
                }
                last_touched_edge_index = Some(index);
                last_touched_edge_start = Some(contour_segment_start.clone());
            }
            Relation::Disjoint => {}
            _ => unreachable!(),
        }
    }
    if !has_no_touch
        && has_no_cross
        && unsafe {
            debug_assert!(last_touched_edge_index.is_some());
            last_touched_edge_index.unwrap_unchecked()
        } == contour_segments.len() - 1
    {
        let (first_contour_segment_start, first_contour_segment_end) =
            contour_segments[0].endpoints();
        if first_contour_segment_start.ne(start)
            && first_contour_segment_start.ne(end)
            && first_contour_segment_end.ne(start)
            && first_contour_segment_end.ne(end)
            && start.orient(end, first_contour_segment_start)
                == Orientation::Collinear
            && point_vertex_line_divides_angle(
                start,
                first_contour_segment_start,
                first_contour_segment_end,
                unsafe {
                    debug_assert!(last_touched_edge_start.is_some());
                    &last_touched_edge_start.unwrap_unchecked()
                },
            )
        {
            has_no_cross = false
        }
    }
    if has_no_cross {
        if has_no_touch {
            Relation::Disjoint
        } else {
            Relation::Touch
        }
    } else {
        Relation::Cross
    }
}

pub(crate) fn relate_to_multisegment<
    'a,
    Multisegment,
    Point: Eq + Hash + Ord,
    Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd,
    Segment: 'a,
>(
    mut start: &'a Point,
    mut end: &'a Point,
    multisegment: &'a Multisegment,
) -> Relation
where
    &'a Multisegment: Multisegmental<IntoIteratorSegment = &'a Segment>,
    &'a Segment: Segmental<Endpoint = &'a Point>,
    for<'b> &'b MultisegmentalIndexSegment<&'a Multisegment>: Segmental,
    for<'b> &'b Point: CrossMultiply<Output = Scalar>
        + Elemental<Coordinate = &'b Scalar>
        + Orient,
{
    let mut has_no_cross = true;
    let mut has_no_touch = true;
    let mut has_no_overlap = true;
    let mut is_segment_superset = true;
    let mut clockwise_middle_touch_scales = Vec::<Scalar>::new();
    let mut counterclockwise_middle_touch_scales = Vec::<Scalar>::new();
    let mut components = Vec::<(&Point, &Point)>::new();
    if start > end {
        (start, end) = (end, start);
    }
    let (original_start, original_end) = (start, end);
    for multisegment_segment in multisegment.segments() {
        let (multisegment_segment_start, multisegment_segment_end) =
            multisegment_segment.endpoints();
        let relation = relate_to_segment(
            original_start,
            original_end,
            multisegment_segment_start,
            multisegment_segment_end,
        );
        if relation == Relation::Component || relation == Relation::Equal {
            return Relation::Component;
        } else if relation == Relation::Composite {
            if has_no_overlap {
                has_no_overlap = false;
            }
            if multisegment_segment_start.eq(start)
                || multisegment_segment_end.eq(start)
            {
                start =
                    multisegment_segment_start.max(multisegment_segment_end);
            } else if multisegment_segment_start.eq(end)
                || multisegment_segment_end.eq(end)
            {
                end = multisegment_segment_start.min(multisegment_segment_end);
            } else {
                components.push(to_sorted_pair((
                    multisegment_segment_start,
                    multisegment_segment_end,
                )));
            }
        } else if relation == Relation::Overlap {
            if is_segment_superset {
                is_segment_superset = false;
            }
            if has_no_overlap {
                has_no_overlap = false
            }
            (start, end) = subtract_segments_overlap(
                start,
                end,
                multisegment_segment_start,
                multisegment_segment_end,
            );
        } else {
            if is_segment_superset {
                is_segment_superset = false;
            }
            if has_no_overlap {
                if relation == Relation::Touch {
                    if has_no_touch {
                        has_no_touch = false;
                    }
                    if has_no_cross
                        && multisegment_segment_start.ne(original_start)
                        && multisegment_segment_end.ne(original_start)
                        && multisegment_segment_start.ne(original_end)
                        && multisegment_segment_end.ne(original_end)
                    {
                        let intersection_scale =
                            to_segments_intersection_scale(
                                original_start,
                                original_end,
                                multisegment_segment_start,
                                multisegment_segment_end,
                            );
                        let non_touched_endpoint = if is_point_in_segment(
                            multisegment_segment_end,
                            original_start,
                            original_end,
                        ) {
                            multisegment_segment_start
                        } else {
                            multisegment_segment_end
                        };
                        if original_start
                            .orient(original_end, non_touched_endpoint)
                            == Orientation::Counterclockwise
                        {
                            &mut counterclockwise_middle_touch_scales
                        } else {
                            &mut clockwise_middle_touch_scales
                        }
                        .push(intersection_scale);
                    }
                } else if has_no_cross && relation == Relation::Cross {
                    has_no_cross = false;
                }
            }
        }
    }
    if has_no_overlap {
        if has_no_cross
            && !clockwise_middle_touch_scales.is_empty()
            && !counterclockwise_middle_touch_scales.is_empty()
        {
            let (less_scales, more_scales) = if clockwise_middle_touch_scales
                .len()
                < counterclockwise_middle_touch_scales.len()
            {
                (
                    clockwise_middle_touch_scales,
                    counterclockwise_middle_touch_scales,
                )
            } else {
                (
                    counterclockwise_middle_touch_scales,
                    clockwise_middle_touch_scales,
                )
            };
            let more_scales_set =
                more_scales.into_iter().collect::<HashSet<Scalar>>();
            if less_scales
                .into_iter()
                .any(|scale| more_scales_set.contains(&scale))
            {
                has_no_cross = false
            }
        }
        if has_no_touch && has_no_cross {
            Relation::Disjoint
        } else {
            if has_no_cross {
                Relation::Touch
            } else {
                Relation::Cross
            }
        }
    } else if !components.is_empty() {
        let (mut min_component_start, mut max_component_end) = components[0];
        for (component_start, component_end) in components[1..].iter().copied()
        {
            if min_component_start > component_start {
                min_component_start = component_start;
            }
            if max_component_end < component_end {
                max_component_end = component_end;
            }
        }
        let components_starts = components
            .iter()
            .copied()
            .map(|(component_start, _)| component_start)
            .collect::<HashSet<&Point>>();
        if min_component_start.eq(start)
            && max_component_end.eq(end)
            && components.into_iter().all(|(_, component_end)| {
                components_starts.contains(component_end)
                    || component_end.eq(max_component_end)
            })
        {
            if is_segment_superset {
                Relation::Equal
            } else {
                Relation::Component
            }
        } else {
            if is_segment_superset {
                Relation::Composite
            } else {
                Relation::Overlap
            }
        }
    } else {
        if start == end {
            if is_segment_superset {
                Relation::Equal
            } else {
                Relation::Component
            }
        } else {
            if is_segment_superset {
                Relation::Composite
            } else {
                Relation::Overlap
            }
        }
    }
}

pub(crate) fn relate_to_segment<Point: PartialOrd>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Relation
where
    for<'a> &'a Point: Orient,
{
    let (first_start, first_end) = to_sorted_pair((first_start, first_end));
    let (second_start, second_end) =
        to_sorted_pair((second_start, second_end));
    if first_start == second_start && first_end == second_end {
        return Relation::Equal;
    }
    let second_start_orientation = first_end.orient(first_start, second_start);
    let second_end_orientation = first_end.orient(first_start, second_end);
    if second_start_orientation == second_end_orientation {
        if second_start_orientation != Orientation::Collinear {
            Relation::Disjoint
        } else if first_start == second_start {
            if second_end < first_end {
                Relation::Composite
            } else {
                Relation::Component
            }
        } else if first_end == second_end {
            if second_start < first_start {
                Relation::Component
            } else {
                Relation::Composite
            }
        } else if second_start == first_end || second_end == first_start {
            Relation::Touch
        } else if first_start < second_start && second_start < first_end {
            if second_end < first_end {
                Relation::Composite
            } else {
                Relation::Overlap
            }
        } else if second_start < first_start && first_start < second_end {
            if first_end < second_end {
                Relation::Component
            } else {
                Relation::Overlap
            }
        } else {
            Relation::Disjoint
        }
    } else if second_start_orientation == Orientation::Collinear {
        if first_start <= second_start && second_start <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if second_end_orientation == Orientation::Collinear {
        if first_start <= second_end && second_end <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else {
        let first_start_orientation =
            second_start.orient(second_end, first_start);
        let first_end_orientation = second_start.orient(second_end, first_end);
        if first_start_orientation == first_end_orientation {
            debug_assert_ne!(first_start_orientation, Orientation::Collinear);
            Relation::Disjoint
        } else if first_start_orientation == Orientation::Collinear {
            if second_start < first_start && first_start < second_end {
                Relation::Touch
            } else {
                Relation::Disjoint
            }
        } else if first_end_orientation == Orientation::Collinear {
            if second_start < first_end && first_end < second_end {
                Relation::Touch
            } else {
                Relation::Disjoint
            }
        } else {
            Relation::Cross
        }
    }
}
