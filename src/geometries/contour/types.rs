use crate::geometries::Point;
use crate::operations::to_arg_min;

#[derive(Clone)]
pub struct Contour<Scalar> {
    pub(super) vertices: Vec<Point<Scalar>>,
}

impl<Scalar: Clone> Contour<Scalar> {
    pub fn new(vertices: Vec<Point<Scalar>>) -> Self {
        Self { vertices }
    }
}

impl<Scalar: Ord> Contour<Scalar> {
    pub(super) fn to_min_vertex_index(&self) -> usize {
        unsafe { to_arg_min(&self.vertices).unwrap_unchecked() }
    }
}
