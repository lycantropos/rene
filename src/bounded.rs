pub struct BoundingBox<Scalar> {
    max_x: Scalar,
    max_y: Scalar,
    min_x: Scalar,
    min_y: Scalar,
}

impl<Scalar> BoundingBox<Scalar> {
    pub fn new(max_x: Scalar, max_y: Scalar, min_x: Scalar, min_y: Scalar) -> Self {
        Self {
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }
}

pub trait Bounded<Scalar> {
    fn to_max_x(&self) -> Scalar;

    fn to_max_y(&self) -> Scalar;

    fn to_min_x(&self) -> Scalar;

    fn to_min_y(&self) -> Scalar;

    fn to_bounding_box(&self) -> BoundingBox<Scalar>;
}
