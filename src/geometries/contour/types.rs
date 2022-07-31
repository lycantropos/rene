use crate::geometries::{Point, Segment};
use crate::operations::to_arg_min;
use crate::traits;

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

impl<Scalar: Clone> traits::Contour for Contour<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;

    fn vertices(&self) -> Vec<Self::Point> {
        self.vertices.clone()
    }

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.vertices.len());
        for index in 0..self.vertices.len() - 1 {
            result.push(Segment::new(
                self.vertices[index].clone(),
                self.vertices[index + 1].clone(),
            ))
        }
        result.push(Segment::new(
            self.vertices[self.vertices.len() - 1].clone(),
            self.vertices[0].clone(),
        ));
        result
    }
}
