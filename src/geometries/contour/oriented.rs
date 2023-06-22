use crate::geometries::Point;
use crate::operations::Orient;
use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<'a, Scalar: Ord> Oriented for &'a Contour<Scalar>
where
    &'a Point<Scalar>: Orient,
{
    fn to_orientation(self) -> Orientation {
        let min_vertex_index = self.to_min_vertex_index();
        let previous_to_min_vertex_index = if min_vertex_index == 0 {
            self.vertices.len() - 1
        } else {
            min_vertex_index - 1
        };
        let next_to_min_vertex_index = unsafe {
            (min_vertex_index + 1)
                .checked_rem_euclid(self.vertices.len())
                .unwrap_unchecked()
        };
        self.vertices[previous_to_min_vertex_index].orient(
            &self.vertices[min_vertex_index],
            &self.vertices[next_to_min_vertex_index],
        )
    }
}

impl<Scalar: Ord> Oriented for Contour<Scalar>
where
    for<'a> &'a Point<Scalar>: Orient,
{
    fn to_orientation(self) -> Orientation {
        let min_vertex_index = self.to_min_vertex_index();
        let previous_to_min_vertex_index = if min_vertex_index == 0 {
            self.vertices.len() - 1
        } else {
            min_vertex_index - 1
        };
        let next_to_min_vertex_index = unsafe {
            (min_vertex_index + 1)
                .checked_rem_euclid(self.vertices.len())
                .unwrap_unchecked()
        };
        self.vertices[previous_to_min_vertex_index].orient(
            &self.vertices[min_vertex_index],
            &self.vertices[next_to_min_vertex_index],
        )
    }
}
