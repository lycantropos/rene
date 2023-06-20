use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::traits::Elemental;

use super::types::Point;

impl<Digit, const SHIFT: usize> Elemental for Point<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Coordinate = Fraction<BigInt<Digit, SHIFT>>;

    fn coordinates(&self) -> (Self::Coordinate, Self::Coordinate) {
        (self.x.clone(), self.y.clone())
    }

    fn x(&self) -> Self::Coordinate {
        self.x.clone()
    }

    fn y(&self) -> Self::Coordinate {
        self.y.clone()
    }
}
