use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits::Multivertexal;

use super::types::Contour;

impl<'a, Digit, const SHIFT: usize> Multivertexal for &'a Contour<Fraction<BigInt<Digit, SHIFT>>> {
    type Vertex = &'a Point<Fraction<BigInt<Digit, SHIFT>>>;
    type Vertices = std::slice::Iter<'a, Point<Fraction<BigInt<Digit, SHIFT>>>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices.iter()
    }

    fn vertices_count(self) -> usize {
        self.vertices.len()
    }
}

impl<Digit, const SHIFT: usize> Multivertexal for Contour<Fraction<BigInt<Digit, SHIFT>>> {
    type Vertex = Point<Fraction<BigInt<Digit, SHIFT>>>;
    type Vertices = std::vec::IntoIter<Point<Fraction<BigInt<Digit, SHIFT>>>>;

    fn vertices(self) -> Self::Vertices {
        self.vertices.into_iter()
    }

    fn vertices_count(self) -> usize {
        self.vertices.len()
    }
}
