use crate::geometries::Point;
use crate::traits::Multivertexal;

use super::types::Contour;

impl<'a, Scalar> Multivertexal for &'a Contour<Scalar> {
    type Vertex = &'a Point<Scalar>;
    type Vertices = std::slice::Iter<'a, Point<Scalar>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices.iter()
    }

    fn vertices_count(self) -> usize {
        self.vertices.len()
    }
}

impl<Scalar> Multivertexal for Contour<Scalar> {
    type Vertex = Point<Scalar>;
    type Vertices = std::vec::IntoIter<Point<Scalar>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices.into_iter()
    }

    fn vertices_count(self) -> usize {
        self.vertices.len()
    }
}
