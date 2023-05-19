use crate::geometries::{Point, Segment};
use crate::locatable::{Locatable, Location};

use super::types::Multisegment;

impl<Scalar> Locatable<&Point<Scalar>> for &Multisegment<Scalar>
where
    for<'a> &'a Segment<Scalar>: Locatable<&'a Point<Scalar>>,
{
    fn locate(self, point: &Point<Scalar>) -> Location {
        self.segments
            .iter()
            .find_map(|segment| {
                let location = segment.locate(point);
                if location == Location::Exterior {
                    None
                } else {
                    Some(location)
                }
            })
            .unwrap_or(Location::Exterior)
    }
}
