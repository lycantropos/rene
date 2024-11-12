use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Div;

use crate::locatable::Location;
use crate::operations::{
    is_point_in_segment, point_vertex_line_divides_angle,
    to_segments_intersection_scale, to_sorted_pair, CrossMultiply, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits::{
    Elemental, Iterable, Lengthsome, Multisegmental, Segmental, Sequence,
};

pub(crate) fn relate_to_contour_segments<
    'a,
    Point: Clone + PartialOrd,
    Segment: 'a,
>(
    (start, end): (&Point, &Point),
    contour_segments: impl Iterator<Item = &'a Segment>,
) -> Relation
where
    for<'b> &'b Point: Orient,
    for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
{
    let mut has_no_cross = true;
    let mut has_no_touch = true;
    let mut last_touched_edge_index: Option<usize> = None;
    let mut last_touched_edge_start: Option<Point> = None;
    let mut contour_segments = contour_segments.enumerate();
    let (mut index, mut contour_segment) =
        unsafe { contour_segments.next().unwrap_unchecked() };
    let (first_contour_segment_start, first_contour_segment_end) =
        contour_segment.endpoints();
    loop {
        let (contour_segment_start, contour_segment_end) =
            contour_segment.endpoints();
        let relation = relate_to_segment_endpoints(
            (start, end),
            (contour_segment_start, contour_segment_end),
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
        if let Some((next_index, next_contour_segment)) =
            contour_segments.next()
        {
            index = next_index;
            contour_segment = next_contour_segment;
        } else {
            break;
        }
    }
    if !has_no_touch
        && has_no_cross
        && unsafe {
            debug_assert!(last_touched_edge_index.is_some());
            last_touched_edge_index.unwrap_unchecked()
        } == index
        && {
            first_contour_segment_start.ne(start)
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
        }
    {
        has_no_cross = false
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

pub(super) fn relate_to_multisegment_segments<
    'a,
    Point: Hash + Ord,
    Scalar: Div<Output = Scalar> + Eq + Hash + PartialOrd,
    Segment: 'a,
>(
    (mut start, mut end): (&'a Point, &'a Point),
    multisegment_segments: impl Iterator<Item = &'a Segment>,
) -> Relation
where
    &'a Segment: Segmental<Endpoint = &'a Point>,
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
    for multisegment_segment in multisegment_segments {
        let (multisegment_segment_start, multisegment_segment_end) =
            multisegment_segment.endpoints();
        let relation = relate_to_segment_endpoints(
            (original_start, original_end),
            (multisegment_segment_start, multisegment_segment_end),
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
        } else if has_no_cross {
            Relation::Touch
        } else {
            Relation::Cross
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
        } else if is_segment_superset {
            Relation::Composite
        } else {
            Relation::Overlap
        }
    } else if start == end {
        if is_segment_superset {
            Relation::Equal
        } else {
            Relation::Component
        }
    } else if is_segment_superset {
        Relation::Composite
    } else {
        Relation::Overlap
    }
}

pub(crate) fn relate_to_segment_endpoints<Point: PartialOrd>(
    first: (&Point, &Point),
    second: (&Point, &Point),
) -> Relation
where
    for<'a> &'a Point: Orient,
{
    let (first_start, first_end) = to_sorted_pair(first);
    let (second_start, second_end) = to_sorted_pair(second);
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

pub(super) fn relate_to_region<
    const REVERSE_ORIENTATION: bool,
    Border,
    Point: PartialOrd,
    Scalar: PartialOrd,
    Segment,
>(
    (start, end): (&Point, &Point),
    border: &Border,
) -> Relation
where
    for<'a> &'a Border: Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar> + Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    let edges = border.segments();
    let relation_with_border = relate_to_region_border((start, end), &edges);
    if relation_with_border == Relation::Cross
        || relation_with_border == Relation::Component
    {
        return relation_with_border;
    }
    let (start_edge_index, start_location) =
        indexed_locate_point_in_region(&edges, start);
    if relation_with_border == Relation::Disjoint {
        if start_location == Location::Exterior {
            Relation::Disjoint
        } else {
            debug_assert_eq!(start_location, Location::Interior);
            Relation::Within
        }
    } else if start_location == Location::Exterior {
        Relation::Touch
    } else if start_location == Location::Interior {
        Relation::Enclosed
    } else {
        let (end_edge_index, end_location) =
            indexed_locate_point_in_region(&edges, end);
        if end_location == Location::Exterior {
            return Relation::Touch;
        } else if end_location == Location::Interior {
            return Relation::Enclosed;
        } else {
            let border_orientation = if REVERSE_ORIENTATION {
                Orientation::Clockwise
            } else {
                Orientation::Counterclockwise
            };
            let (edge_start, edge_end) = edges[start_edge_index].endpoints();
            if start == edge_start {
                let prev_start = if REVERSE_ORIENTATION {
                    edges[(start_edge_index + 1) % edges.len()].end()
                } else {
                    edges[if start_edge_index == 0 {
                        edges.len() - 1
                    } else {
                        start_edge_index - 1
                    }]
                    .start()
                };
                if prev_start.orient(edge_start, edge_end)
                    == border_orientation
                {
                    if (edge_start.orient(prev_start, end)
                        == border_orientation)
                        || (edge_end.orient(edge_start, end)
                            == border_orientation)
                    {
                        return Relation::Touch;
                    }
                } else if edge_start.orient(prev_start, end)
                    == border_orientation
                    && edge_end.orient(edge_start, end) == border_orientation
                {
                    return Relation::Touch;
                }
            } else if start == edge_end {
                let next_end = if REVERSE_ORIENTATION {
                    edges[if start_edge_index == 0 {
                        edges.len() - 1
                    } else {
                        start_edge_index - 1
                    }]
                    .start()
                } else {
                    edges[(start_edge_index + 1) % edges.len()].end()
                };
                if edge_start.orient(edge_end, next_end) == border_orientation
                    && (edge_end.orient(edge_start, end) == border_orientation
                        || next_end.orient(edge_end, end)
                            == border_orientation)
                {
                    return Relation::Touch;
                }
            } else if edge_end.orient(edge_start, end) == border_orientation {
                return Relation::Touch;
            }
            let (edge_start, edge_end) = edges[end_edge_index].endpoints();
            if end == edge_start {
                let prev_start = if REVERSE_ORIENTATION {
                    edges[(end_edge_index + 1) % edges.len()].end()
                } else {
                    edges[if end_edge_index == 0 {
                        edges.len() - 1
                    } else {
                        end_edge_index - 1
                    }]
                    .start()
                };
                if prev_start.orient(edge_start, edge_end)
                    == border_orientation
                {
                    if (edge_start.orient(prev_start, start)
                        == border_orientation)
                        || (edge_end.orient(edge_start, start)
                            == border_orientation)
                    {
                        return Relation::Touch;
                    }
                } else if edge_start.orient(prev_start, start)
                    == border_orientation
                    && edge_end.orient(edge_start, start) == border_orientation
                {
                    return Relation::Touch;
                }
            } else if end == edge_end {
                let next_end = if REVERSE_ORIENTATION {
                    edges[if end_edge_index == 0 {
                        edges.len() - 1
                    } else {
                        end_edge_index - 1
                    }]
                    .start()
                } else {
                    edges[(end_edge_index + 1) % edges.len()].end()
                };
                if edge_start.orient(edge_end, next_end) == border_orientation
                {
                    if (edge_end.orient(edge_start, start)
                        == border_orientation)
                        || (next_end.orient(edge_end, start)
                            == border_orientation)
                    {
                        return Relation::Touch;
                    }
                } else if edge_end.orient(edge_start, start)
                    == border_orientation
                    && next_end.orient(edge_end, start) == border_orientation
                {
                    return Relation::Touch;
                }
            } else if edge_end.orient(edge_start, start) == border_orientation
            {
                return Relation::Touch;
            }
        }
        Relation::Enclosed
    }
}

fn relate_to_region_border<Edges, Point: PartialOrd, Segment>(
    (start, end): (&Point, &Point),
    border_edges: &Edges,
) -> Relation
where
    Edges: Sequence<IndexItem = Segment>,
    for<'a> &'a Point: Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    /*
        similar to segment-in-contour check
        but cross has higher priority over overlap
        because cross with border will be considered as cross with region
        whereas overlap with border can't be an overlap with region
        && should be classified by further analysis
    */
    let mut has_no_touch = true;
    let mut has_no_overlap = true;
    let mut last_touched_edge_start = None;
    let mut last_touched_edge_index = usize::MAX;
    for (index, edge) in border_edges.iter().enumerate() {
        let (edge_start, edge_end) = edge.endpoints();
        let relation_with_edge =
            relate_to_segment_endpoints((start, end), (edge_start, edge_end));
        if relation_with_edge == Relation::Component
            || relation_with_edge == Relation::Equal
        {
            return Relation::Component;
        } else if relation_with_edge == Relation::Overlap
            || relation_with_edge == Relation::Composite
        {
            if has_no_overlap {
                has_no_overlap = false;
            }
        } else if relation_with_edge == Relation::Touch {
            if has_no_touch {
                has_no_touch = false
            } else if index - 1 == last_touched_edge_index
                && start != edge_start
                && start != edge_end
                && end != edge_start
                && end != edge_end
                && (start.orient(end, edge_start) == Orientation::Collinear)
                && point_vertex_line_divides_angle(
                    start,
                    edge_start,
                    unsafe { last_touched_edge_start.unwrap_unchecked() },
                    edge_end,
                )
            {
                return Relation::Cross;
            }
            last_touched_edge_index = index;
            last_touched_edge_start = Some(edge_start);
        } else if relation_with_edge == Relation::Cross {
            return Relation::Cross;
        }
    }
    if !has_no_touch && last_touched_edge_index == border_edges.len() - 1 {
        let (first_edge_start, first_edge_end) = border_edges[0].endpoints();
        if (relate_to_segment_endpoints(
            (first_edge_start, first_edge_end),
            (start, end),
        ) == Relation::Touch)
            && start != first_edge_start
            && start != first_edge_end
            && end != first_edge_start
            && end != first_edge_end
            && (start.orient(end, first_edge_start) == Orientation::Collinear)
            && {
                point_vertex_line_divides_angle(
                    start,
                    first_edge_start,
                    unsafe { last_touched_edge_start.unwrap_unchecked() },
                    first_edge_end,
                )
            }
        {
            return Relation::Cross;
        }
    }
    if has_no_overlap {
        if has_no_touch {
            Relation::Disjoint
        } else {
            Relation::Touch
        }
    } else {
        Relation::Overlap
    }
}

fn indexed_locate_point_in_region<Edges, Point: PartialEq, Scalar, Segment>(
    border_edges: &Edges,
    point: &Point,
) -> (usize, Location)
where
    Scalar: PartialOrd,
    Edges: Sequence<IndexItem = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar> + Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    let mut result = false;
    let point_y = point.y();
    for (index, edge) in border_edges.iter().enumerate() {
        let (start, end) = edge.endpoints();
        if is_point_in_segment(point, start, end) {
            return (index, Location::Boundary);
        }
        let start_y = start.y();
        let end_y = end.y();
        if (start_y.gt(point_y)) != (end_y.gt(point_y))
            && ((end_y.gt(start_y))
                == (start.orient(end, point) == Orientation::Counterclockwise))
        {
            result = !result;
        }
    }
    (
        usize::MAX,
        if result {
            Location::Interior
        } else {
            Location::Exterior
        },
    )
}

fn subtract_segments_overlap<'a, Point: PartialOrd>(
    minuend_start: &'a Point,
    minuend_end: &'a Point,
    subtrahend_start: &'a Point,
    subtrahend_end: &'a Point,
) -> (&'a Point, &'a Point) {
    let (minuend_start, minuend_end) =
        to_sorted_pair((minuend_start, minuend_end));
    let (subtrahend_start, subtrahend_end) =
        to_sorted_pair((subtrahend_start, subtrahend_end));
    if subtrahend_start < minuend_start && minuend_start < subtrahend_end {
        (subtrahend_end, minuend_end)
    } else {
        (minuend_start, subtrahend_start)
    }
}
