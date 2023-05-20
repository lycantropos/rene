use crate::geometries::{Contour, Point};
use crate::locatable::{Locatable, Location};
use crate::operations::{locate_point_in_region, Orient};
use crate::traits::{Elemental, Multisegmental, Segmental};

use super::types::Polygon;

impl<Scalar: PartialOrd> Locatable<&Point<Scalar>> for &Polygon<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar> + Orient,
    Contour<Scalar>: Multisegmental,
    <Contour<Scalar> as Multisegmental>::Segment: Segmental<Endpoint = Point<Scalar>>,
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
