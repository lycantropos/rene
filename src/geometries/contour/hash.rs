use std::hash::{Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<Scalar: AdditiveGroup + Clone + Hash + MultiplicativeMonoid + Ord + Signed> Hash
    for Contour<Scalar>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let min_vertex_index = self.to_min_vertex_index();
        self.0[min_vertex_index].hash(state);
        match self.orientation() {
            Orientation::Clockwise => {
                for index in (0..min_vertex_index).rev() {
                    self.0[index].hash(state);
                }
                for index in (min_vertex_index + 1..self.0.len()).rev() {
                    self.0[index].hash(state);
                }
            }
            _ => {
                for index in min_vertex_index + 1..self.0.len() {
                    self.0[index].hash(state);
                }
                for index in 0..min_vertex_index {
                    self.0[index].hash(state);
                }
            }
        }
    }
}
