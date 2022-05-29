use super::types::Point;

impl<Scalar> From<(Scalar, Scalar)> for Point<Scalar> {
    fn from((x, y): (Scalar, Scalar)) -> Self {
        Self::new(x, y)
    }
}
