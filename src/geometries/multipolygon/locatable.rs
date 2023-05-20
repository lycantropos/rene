use crate::geometries::{Point, Polygon};
use crate::locatable::{Locatable, Location};

use super::types::Multipolygon;

impl<Scalar> Locatable<&Point<Scalar>> for &Multipolygon<Scalar>
where
    for<'a> &'a Polygon<Scalar>: Locatable<&'a Point<Scalar>>,
{
    fn locate(self, point: &Point<Scalar>) -> Location {
        self.polygons
            .iter()
            .find_map(|polygon| {
                let location = polygon.locate(point);
                if location == Location::Exterior {
                    None
                } else {
                    Some(location)
                }
            })
            .unwrap_or(Location::Exterior)
    }
}
