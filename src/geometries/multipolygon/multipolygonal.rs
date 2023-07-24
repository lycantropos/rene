use crate::geometries::Polygon;
use crate::traits::Multipolygonal;

use super::types::Multipolygon;

impl<'a, Scalar> Multipolygonal for &'a Multipolygon<Scalar> {
    type Polygon = &'a Polygon<Scalar>;
    type Polygons = std::slice::Iter<'a, Polygon<Scalar>>;

    fn polygons(self) -> Self::Polygons {
        self.polygons.iter()
    }

    fn polygons_count(self) -> usize {
        self.polygons.len()
    }
}

impl<Scalar> Multipolygonal for Multipolygon<Scalar> {
    type Polygon = Polygon<Scalar>;
    type Polygons = std::vec::IntoIter<Polygon<Scalar>>;

    fn polygons(self) -> Self::Polygons {
        self.polygons.into_iter()
    }

    fn polygons_count(self) -> usize {
        self.polygons.len()
    }
}
