use std::hash::{Hash, Hasher};

use crate::oriented::{Orientation, Oriented};

use super::types::Contour;

impl<Scalar: Hash + Ord> Hash for Contour<Scalar>
where
    for<'a> &'a Self: Oriented,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let min_vertex_index = self.to_min_vertex_index();
        self.vertices[min_vertex_index].hash(state);
        if self.to_orientation() == Orientation::Clockwise {
            for vertex in self.vertices.iter().take(min_vertex_index).rev() {
                vertex.hash(state);
            }
            for vertex in self.vertices.iter().skip(min_vertex_index + 1).rev()
            {
                vertex.hash(state);
            }
        } else {
            for vertex in self.vertices.iter().skip(min_vertex_index + 1) {
                vertex.hash(state);
            }
            for vertex in self.vertices.iter().take(min_vertex_index) {
                vertex.hash(state);
            }
        }
    }
}
