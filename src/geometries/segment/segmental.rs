use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits::Segmental;

use super::types::Segment;

impl<'a, Digit, const SHIFT: usize> Segmental for &'a Segment<Fraction<BigInt<Digit, SHIFT>>> {
    type Endpoint = &'a Point<Fraction<BigInt<Digit, SHIFT>>>;

    fn start(self) -> Self::Endpoint {
        &self.start
    }

    fn end(self) -> Self::Endpoint {
        &self.end
    }

    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint) {
        (&self.start, &self.end)
    }
}

impl<Digit, const SHIFT: usize> Segmental for Segment<Fraction<BigInt<Digit, SHIFT>>> {
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
