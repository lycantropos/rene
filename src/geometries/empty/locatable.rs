use crate::geometries::Point;
use crate::locatable::{Locatable, Location};

use super::types::Empty;

impl<Scalar> Locatable<&Point<Scalar>> for &Empty {
    fn locate(self, _point: &Point<Scalar>) -> Location {
        Location::Exterior
    }
}
