use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Sign, Signed};

use crate::geometries::operations;
use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + Ord + Signed> Oriented
    for Contour<Scalar>
{
    fn orientation(&self) -> Orientation {
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
        match operations::cross_multiply(
            self.vertices[previous_to_min_vertex_index].clone(),
            self.vertices[min_vertex_index].clone(),
            self.vertices[previous_to_min_vertex_index].clone(),
            self.vertices[next_to_min_vertex_index].clone(),
        )
        .sign()
        {
            Sign::Negative => Orientation::Clockwise,
            Sign::Positive => Orientation::Counterclockwise,
            Sign::Zero => Orientation::Collinear,
        }
    }
}
