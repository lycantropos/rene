use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits::Multivertexal;

use super::types::Contour;

impl<Digit, const SHIFT: usize> Multivertexal for &Contour<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Vertex = Point<Fraction<BigInt<Digit, SHIFT>>>;
    type Vertices = std::vec::IntoIter<Self::Vertex>;

    fn vertices(self) -> Self::Vertices {
        self.vertices.clone().into_iter()
    }

    fn vertices_count(self) -> usize {
        self.vertices.len()
    }
}
