use crate::geometries::{Point, Segment};
use crate::operations::to_arg_min;

#[derive(Clone)]
pub struct Contour<Scalar> {
    pub(super) segments: Vec<Segment<Scalar>>,
    pub(super) vertices: Vec<Point<Scalar>>,
}

impl<Scalar> Contour<Scalar>
where
    Point<Scalar>: Clone,
{
    #[must_use]
    pub fn new(vertices: Vec<Point<Scalar>>) -> Self {
        let mut segments =
            Vec::<Segment<Scalar>>::with_capacity(vertices.len());
        for index in 0..vertices.len() - 1 {
            segments.push(Segment::new(
                vertices[index].clone(),
                vertices[index + 1].clone(),
            ));
        }
        segments.push(Segment::new(
            vertices[vertices.len() - 1].clone(),
            vertices[0].clone(),
        ));
        Self { segments, vertices }
    }
}

impl<Scalar: Ord> Contour<Scalar> {
    pub(super) fn to_min_vertex_index(&self) -> usize {
        unsafe { to_arg_min(&self.vertices).unwrap_unchecked() }
    }
}
