use crate::traits::Elemental;

use super::types::Point;

impl<'a, Scalar> Elemental for &'a Point<Scalar> {
    type Coordinate = &'a Scalar;

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

impl<Scalar> Elemental for Point<Scalar> {
    type Coordinate = Scalar;

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
