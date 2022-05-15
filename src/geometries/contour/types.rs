use crate::geometries::{Point, Segment};
use crate::traits;

#[derive(Clone)]
pub struct Contour<Scalar>(pub(in crate::geometries) Vec<Point<Scalar>>);

impl<Scalar: Clone> Contour<Scalar> {
    pub fn new(vertices: Vec<Point<Scalar>>) -> Self {
        Self(vertices)
    }
}

impl<Scalar: Ord> Contour<Scalar> {
    pub(super) fn to_min_vertex_index(&self) -> usize {
        unsafe {
            (0..self.0.len())
                .min_by_key(|index| &self.0[*index])
                .unwrap_unchecked()
        }
    }
}

impl<Scalar: Clone> traits::Contour<Scalar> for Contour<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;

    fn vertices(&self) -> Vec<Self::Point> {
        self.0.clone()
    }

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.0.len());
        for index in 0..self.0.len() - 1 {
            result.push(Segment::new(
                self.0[index].clone(),
                self.0[index + 1].clone(),
            ))
        }
        result.push(Segment::new(
            self.0[self.0.len() - 1].clone(),
            self.0[0].clone(),
        ));
        result
    }
}
