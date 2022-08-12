use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Contour
    for Contour<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Point = self::Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;
    type Segment = self::Segment<<Self::Point as traits::Point>::Coordinate>;

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

    fn segments_count(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> Vec<Self::Point> {
        self.vertices.clone()
    }

    fn vertices_count(&self) -> usize {
        self.vertices.len()
    }
}
