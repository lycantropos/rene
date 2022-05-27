#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    Clockwise,
    Collinear,
    Counterclockwise,
}

pub trait Oriented {
    fn orientation(self) -> Orientation;
}
