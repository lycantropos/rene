use crate::geometries::Contour;
use crate::slice_sequence::SliceSequence;
use crate::traits::Polygonal2;

use super::types::Polygon;

impl<'a, Scalar> Polygonal2 for &'a Polygon<Scalar> {
    type Contour = &'a Contour<Scalar>;
    type IndexHole = Contour<Scalar>;
    type IntoIteratorHole = &'a Contour<Scalar>;
    type Holes = SliceSequence<'a, Contour<Scalar>>;

    fn border2(self) -> Self::Contour {
        &self.border
    }

    fn holes2(self) -> Self::Holes {
        SliceSequence::new(&self.holes)
    }
}

impl<Scalar> Polygonal2 for Polygon<Scalar> {
    type Contour = Contour<Scalar>;
    type IndexHole = Contour<Scalar>;
    type IntoIteratorHole = Contour<Scalar>;
    type Holes = Vec<Contour<Scalar>>;

    fn border2(self) -> Self::Contour {
        self.border
    }

    fn holes2(self) -> Self::Holes {
        self.holes
    }
}
