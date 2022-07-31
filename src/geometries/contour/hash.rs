use std::hash::{Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<Scalar: AdditiveGroup + Clone + Hash + MultiplicativeMonoid + Ord + Signed> Hash
    for Contour<Scalar>
where
    Self: Oriented,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let min_vertex_index = self.to_min_vertex_index();
        self.vertices[min_vertex_index].hash(state);
        match self.to_orientation() {
            Orientation::Clockwise => {
                for index in (0..min_vertex_index).rev() {
                    self.vertices[index].hash(state);
                }
                for index in (min_vertex_index + 1..self.vertices.len()).rev() {
                    self.vertices[index].hash(state);
                }
            }
            _ => {
                for index in min_vertex_index + 1..self.vertices.len() {
                    self.vertices[index].hash(state);
                }
                for index in 0..min_vertex_index {
                    self.vertices[index].hash(state);
                }
            }
        }
    }
}
