use super::types::Box;

pub trait Bounded<Scalar> {
    fn to_max_x(&self) -> Scalar;

    fn to_max_y(&self) -> Scalar;

    fn to_min_x(&self) -> Scalar;

    fn to_min_y(&self) -> Scalar;

    fn to_bounding_box(&self) -> Box<Scalar> {
        Box::new(
            self.to_min_x(),
            self.to_max_x(),
            self.to_min_y(),
            self.to_max_y(),
        )
    }
}
