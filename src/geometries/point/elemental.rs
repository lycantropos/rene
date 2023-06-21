use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::traits::Elemental;

use super::types::Point;

impl<'a, Digit, const SHIFT: usize> Elemental for &'a Point<Fraction<BigInt<Digit, SHIFT>>>
where
    Fraction<BigInt<Digit, SHIFT>>: Clone,
{
    type Coordinate = &'a Fraction<BigInt<Digit, SHIFT>>;

    fn coordinates(self) -> (Self::Coordinate, Self::Coordinate) {
        (&self.x, &self.y)
    }

    fn x(self) -> Self::Coordinate {
        &self.x
    }

    fn y(self) -> Self::Coordinate {
        &self.y
    }
}

impl<Digit, const SHIFT: usize> Elemental for Point<Fraction<BigInt<Digit, SHIFT>>> {
    type Coordinate = Fraction<BigInt<Digit, SHIFT>>;

    fn coordinates(self) -> (Self::Coordinate, Self::Coordinate) {
        (self.x, self.y)
    }

    fn x(self) -> Self::Coordinate {
        self.x
    }

    fn y(self) -> Self::Coordinate {
        self.y
    }
}
