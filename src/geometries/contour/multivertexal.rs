use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits::Multivertexal;

use super::types::Contour;

impl<Digit, const SHIFT: usize> Multivertexal for Contour<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Vertex = self::Point<Fraction<BigInt<Digit, SHIFT>>>;

    fn vertices(&self) -> Vec<Self::Vertex> {
        self.vertices.clone()
    }

    fn vertices_count(&self) -> usize {
        self.vertices.len()
    }
}
