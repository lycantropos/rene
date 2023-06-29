use crate::geometries::Point;
use crate::locatable::{Locatable, Location};
use crate::operations::{is_point_in_segment, Orient};
use crate::traits::Elemental;

use super::types::Contour;

impl<Scalar: PartialOrd> Locatable<&Point<Scalar>> for &Contour<Scalar>
where
    Point<Scalar>: PartialEq,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar> + Orient,
{
    fn locate(self, point: &Point<Scalar>) -> Location {
        for index in 0..self.vertices.len() - 1 {
            if is_point_in_segment(
                point,
                &self.vertices[index],
                &self.vertices[index + 1],
            ) {
                return Location::Boundary;
            }
        }
        if is_point_in_segment(
            point,
            &self.vertices[self.vertices.len() - 1],
            &self.vertices[0],
        ) {
            Location::Boundary
        } else {
            Location::Exterior
        }
    }
}
