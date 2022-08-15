use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::traits::Elemental;

use super::types::Point;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Elemental
    for Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

    fn x(&self) -> Self::Coordinate {
        self.x.clone()
    }

    fn y(&self) -> Self::Coordinate {
        self.y.clone()
    }
}
