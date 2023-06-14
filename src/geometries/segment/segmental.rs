use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits::Segmental;

use super::types::Segment;

impl<Digit, const SHIFT: usize> Segmental for &Segment<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Endpoint = Point<Fraction<BigInt<Digit, SHIFT>>>;

    fn start(self) -> Self::Endpoint {
        self.start.clone()
    }

    fn end(self) -> Self::Endpoint {
        self.end.clone()
    }

    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint) {
        (self.start.clone(), self.end.clone())
    }
}

impl<Digit, const SHIFT: usize> Segmental for Segment<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Endpoint = Point<Fraction<BigInt<Digit, SHIFT>>>;

    fn start(self) -> Self::Endpoint {
        self.start
    }

    fn end(self) -> Self::Endpoint {
        self.end
    }

    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint) {
        (self.start, self.end)
    }
}
