use crate::geometries::{Contour, Point, Segment};
use crate::locatable::{Locatable, Location};
use crate::operations::{locate_point_in_region, Orient};
use crate::traits::{Elemental, Multisegmental, Segmental};

use super::types::Polygon;

impl<'a, Scalar: PartialOrd> Locatable<&Point<Scalar>> for &'a Polygon<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'b> &'b Contour<Scalar>: Multisegmental<Segment = &'b Segment<Scalar>>,
    for<'b> &'b Point<Scalar>: Elemental<Coordinate = &'b Scalar> + Orient,
    for<'b> &'b Segment<Scalar>: Segmental<Endpoint = &'b Point<Scalar>>,
{
    fn locate(self, point: &Point<Scalar>) -> Location {
        let location_without_holes = locate_point_in_region(&self.border, point);
        if location_without_holes == Location::Interior {
            for hole in &self.holes {
                let location_in_hole = locate_point_in_region(hole, point);
                if location_in_hole == Location::Interior {
                    return Location::Exterior;
                } else if location_in_hole == Location::Boundary {
                    return Location::Boundary;
                }
            }
        }
        location_without_holes
    }
}
