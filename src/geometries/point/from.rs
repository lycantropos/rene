use super::types::Point;

impl<Scalar> From<(Scalar, Scalar)> for Point<Scalar> {
    fn from((x, y): (Scalar, Scalar)) -> Self {
        Self::new(x, y)
    }
}

impl<Scalar> From<[Scalar; 2]> for Point<Scalar> {
    fn from([x, y]: [Scalar; 2]) -> Self {
        Self::new(x, y)
    }
}
