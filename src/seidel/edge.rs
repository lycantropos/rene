use crate::operations::Orient;
use crate::oriented::Orientation;

#[derive(Clone)]
pub(crate) struct Edge {
    pub(super) left_point_index: usize,
    pub(super) right_point_index: usize,
    pub(super) interior_to_left: bool,
}

impl Edge {
    pub(super) fn is_under<Point: PartialEq>(
        &self,
        other: &Self,
        endpoints: &[Point],
    ) -> bool
    where
        for<'a> &'a Point: Orient,
    {
        let other_left_orientation =
            self.orientation_of(&endpoints[other.left_point_index], endpoints);
        let other_right_orientation = self
            .orientation_of(&endpoints[other.right_point_index], endpoints);
        if other_left_orientation == other_right_orientation {
            other_left_orientation == Orientation::Counterclockwise
        } else if other_left_orientation == Orientation::Collinear {
            other_right_orientation == Orientation::Counterclockwise
        } else {
            let left_orientation = other
                .orientation_of(&endpoints[self.left_point_index], endpoints);
            let right_orientation = other
                .orientation_of(&endpoints[self.right_point_index], endpoints);
            if left_orientation == right_orientation {
                left_orientation == Orientation::Clockwise
            } else if left_orientation == Orientation::Collinear {
                right_orientation == Orientation::Clockwise
            } else if other_right_orientation == Orientation::Collinear {
                other_left_orientation == Orientation::Counterclockwise
            } else if right_orientation == Orientation::Collinear {
                left_orientation == Orientation::Clockwise
            } else {
                // crossing edges are incomparable
                false
            }
        }
    }

    pub(super) fn orientation_of<Point>(
        &self,
        point: &Point,
        endpoints: &[Point],
    ) -> Orientation
    where
        for<'a> &'a Point: Orient,
    {
        endpoints[self.left_point_index]
            .orient(&endpoints[self.right_point_index], point)
    }
}
