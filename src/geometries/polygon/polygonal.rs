use crate::geometries::Contour;
use crate::slice_sequence::SliceSequence;
use crate::traits::Polygonal;

use super::types::Polygon;

impl<'a, Scalar> Polygonal for &'a Polygon<Scalar> {
    type Contour = &'a Contour<Scalar>;
    type IndexHole = Contour<Scalar>;
    type IntoIteratorHole = &'a Contour<Scalar>;
    type Holes = SliceSequence<'a, Contour<Scalar>>;

    fn border(self) -> Self::Contour {
        &self.border
    }

    fn components(self) -> (Self::Contour, Self::Holes) {
        (self.border(), self.holes())
    }

    fn holes(self) -> Self::Holes {
        SliceSequence::new(&self.holes)
    }
}

impl<Scalar> Polygonal for Polygon<Scalar> {
    type Contour = Contour<Scalar>;
    type IndexHole = Contour<Scalar>;
    type IntoIteratorHole = Contour<Scalar>;
    type Holes = Vec<Contour<Scalar>>;

    fn border(self) -> Self::Contour {
        self.border
    }

    fn components(self) -> (Self::Contour, Self::Holes) {
        (self.border, self.holes)
    }

    fn holes(self) -> Self::Holes {
        self.holes
    }
}
