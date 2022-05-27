use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::geometries::Point;
use crate::operations;
use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + Ord + Signed> Oriented
    for &Contour<Scalar>
{
    fn orientation(self) -> Orientation {
        let min_vertex_index = self.to_min_vertex_index();
        let previous_to_min_vertex_index = if min_vertex_index.is_zero() {
            self.vertices.len() - 1
        } else {
            min_vertex_index - 1
        };
        let next_to_min_vertex_index = unsafe {
            (min_vertex_index + 1)
                .checked_rem_euclid(self.vertices.len())
                .unwrap_unchecked()
        };
        operations::orient::<Scalar, Point<Scalar>>(
            &self.vertices[previous_to_min_vertex_index],
            &self.vertices[min_vertex_index],
            &self.vertices[next_to_min_vertex_index],
        )
    }
}
