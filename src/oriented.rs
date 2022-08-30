#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Clockwise,
    Collinear,
    Counterclockwise,
}

pub trait Oriented {
    fn to_orientation(&self) -> Orientation;
}
