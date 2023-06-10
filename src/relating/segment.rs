use crate::operations::{to_sorted_pair, Orient};
use crate::oriented::Orientation;
use crate::relatable::Relation;

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
