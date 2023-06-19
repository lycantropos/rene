use crate::operations::{point_vertex_line_divides_angle, to_sorted_pair, Orient};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits::{Contoural, Multisegmental, Multivertexal, Segmental};

pub(crate) fn relate_to_contour<
    'a,
    Contour,
    Point: Orient + PartialOrd,
    Segment: Segmental<Endpoint = Point>,
>(
    start: &Point,
    end: &Point,
    contour: &'a Contour,
) -> Relation
where
    &'a Contour: Contoural<Segment = Segment, Vertex = Point>,
{
    let mut has_no_cross = true;
    let mut has_no_touch = true;
    let mut last_touched_edge_index: Option<usize> = None;
    let mut last_touched_edge_start: Option<Point> = None;
    for (index, contour_segment) in contour.segments().enumerate() {
        let (contour_segment_start, contour_segment_end) = contour_segment.endpoints();
        let relation = relate_to_segment(start, end, &contour_segment_start, &contour_segment_end);
        match relation {
            Relation::Component | Relation::Equal => return Relation::Component,
            Relation::Composite | Relation::Overlap => return Relation::Overlap,
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
                    if index - unsafe { last_touched_edge_index.unwrap_unchecked() } == 1
                        && contour_segment_start.ne(start)
                        && contour_segment_start.ne(end)
                        && contour_segment_end.ne(start)
                        && contour_segment_end.ne(end)
                        && start.orient(end, &contour_segment_start) == Orientation::Collinear
                        && point_vertex_line_divides_angle(
                            start,
                            &contour_segment_start,
                            &contour_segment_end,
                            unsafe { &last_touched_edge_start.unwrap_unchecked() },
                        )
                    {
                        has_no_cross = false;
                    }
                }
                last_touched_edge_index = Some(index);
                last_touched_edge_start = Some(contour_segment_start);
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
        } == contour.segments_count() - 1
    {
        let (first_contour_segment_start, first_contour_segment_end) =
            unsafe { contour.segments().next().unwrap_unchecked() }.endpoints();
        if first_contour_segment_start.ne(start)
            && first_contour_segment_start.ne(end)
            && first_contour_segment_end.ne(start)
            && first_contour_segment_end.ne(end)
            && start.orient(end, &first_contour_segment_start) == Orientation::Collinear
            && point_vertex_line_divides_angle(
                start,
                &first_contour_segment_start,
                &first_contour_segment_end,
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

pub(crate) fn relate_to_segment<Point: Orient + PartialOrd>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Relation {
    let (first_start, first_end) = to_sorted_pair((first_start, first_end));
    let (second_start, second_end) = to_sorted_pair((second_start, second_end));
    let starts_equal = second_start == first_start;
    let ends_equal = second_end == first_end;
    if starts_equal && ends_equal {
        return Relation::Equal;
    }
    let second_start_orientation = first_end.orient(first_start, second_start);
    let second_end_orientation = first_end.orient(first_start, second_end);
    if second_start_orientation != Orientation::Collinear
        && second_end_orientation != Orientation::Collinear
    {
        if second_start_orientation == second_end_orientation {
            Relation::Disjoint
        } else {
            let first_start_orientation = second_start.orient(second_end, first_start);
            let first_end_orientation = second_start.orient(second_end, first_end);
            if first_start_orientation != Orientation::Collinear
                && first_end_orientation != Orientation::Collinear
            {
                if first_start_orientation == first_end_orientation {
                    Relation::Disjoint
                } else {
                    Relation::Cross
                }
            } else if first_start_orientation != Orientation::Collinear {
                if second_start < first_end && first_end < second_end {
                    Relation::Touch
                } else {
                    Relation::Disjoint
                }
            } else if second_start < first_start && first_start < second_end {
                Relation::Touch
            } else {
                Relation::Disjoint
            }
        }
    } else if second_start_orientation != Orientation::Collinear {
        if first_start <= second_end && second_end <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if second_end_orientation != Orientation::Collinear {
        if first_start <= second_start && second_start <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if starts_equal {
        if second_end < first_end {
            Relation::Composite
        } else {
            Relation::Component
        }
    } else if ends_equal {
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
}
