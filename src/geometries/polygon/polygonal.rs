use crate::geometries::Contour;
use crate::traits::Polygonal;

use super::types::Polygon;

impl<'a, Scalar> Polygonal for &'a Polygon<Scalar> {
    type Contour = &'a Contour<Scalar>;
    type Holes = std::slice::Iter<'a, Contour<Scalar>>;

    fn border(self) -> Self::Contour {
        &self.border
    }

    fn holes(self) -> Self::Holes {
        self.holes.iter()
    }

    fn holes_count(self) -> usize {
        self.holes.len()
    }
}

impl<Scalar> Polygonal for Polygon<Scalar> {
    type Contour = Contour<Scalar>;
    type Holes = std::vec::IntoIter<Contour<Scalar>>;

    fn border(self) -> Self::Contour {
        self.border
    }

    fn holes(self) -> Self::Holes {
        self.holes.into_iter()
    }

    fn holes_count(self) -> usize {
        self.holes.len()
    }
}
