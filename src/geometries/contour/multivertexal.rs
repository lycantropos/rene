use crate::geometries::Point;
use crate::slice_sequence::SliceSequence;
use crate::traits::Multivertexal;

use super::types::Contour;

impl<'a, Scalar> Multivertexal for &'a Contour<Scalar> {
    type IndexVertex = Point<Scalar>;
    type IntoIteratorVertex = &'a Point<Scalar>;
    type Vertices = SliceSequence<'a, Point<Scalar>>;

    fn vertices(self) -> Self::Vertices {
        SliceSequence::new(&self.vertices)
    }
}

impl<Scalar> Multivertexal for Contour<Scalar> {
    type IndexVertex = Point<Scalar>;
    type IntoIteratorVertex = Point<Scalar>;
    type Vertices = Vec<Point<Scalar>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices
    }
}
