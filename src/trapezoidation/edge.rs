use std::cmp::Ordering;

use crate::operations::Orient;
use crate::oriented::Orientation;

#[derive(Clone)]
pub(crate) struct Edge<Point> {
    pub(super) left_point: Point,
    pub(super) right_point: Point,
    pub(super) interior_to_left: bool,
}

impl<Point: PartialEq> PartialEq<Self> for Edge<Point> {
    fn eq(&self, other: &Self) -> bool {
        self.left_point.eq(&other.left_point) && self.right_point.eq(&self.right_point) && {
            debug_assert_eq!(self.interior_to_left, other.interior_to_left);
            true
        }
    }
}

impl<Point: PartialEq + Orient> PartialOrd for Edge<Point> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let other_left_orientation = self.orientation_of(&other.left_point);
        let other_right_orientation = self.orientation_of(&other.right_point);
        if other_left_orientation == other_right_orientation {
            return if other_left_orientation == Orientation::Counterclockwise {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            };
        } else if other_left_orientation == Orientation::Collinear {
            return if other_right_orientation == Orientation::Counterclockwise {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            };
        }
        let left_orientation = other.orientation_of(&self.left_point);
        let right_orientation = other.orientation_of(&self.right_point);
        if left_orientation == right_orientation {
            if left_orientation == Orientation::Clockwise {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else if left_orientation == Orientation::Collinear {
            if right_orientation == Orientation::Clockwise {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else if other_right_orientation == Orientation::Collinear {
            if other_left_orientation == Orientation::Counterclockwise {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            if right_orientation == Orientation::Collinear {
                if left_orientation == Orientation::Clockwise {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            } else {
                // crossing edges are incomparable
                None
            }
        }
    }
}

impl<Point: Orient> Edge<Point> {
    pub(super) fn orientation_of(&self, point: &Point) -> Orientation {
        self.left_point.orient(&self.right_point, point)
    }
}
