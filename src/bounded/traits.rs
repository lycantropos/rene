use super::types::Box;

pub trait Bounded<Scalar> {
    fn to_bounding_box(self) -> Box<Scalar>;

    fn to_max_x(self) -> Scalar;

    fn to_max_y(self) -> Scalar;

    fn to_min_x(self) -> Scalar;

    fn to_min_y(self) -> Scalar;
}
