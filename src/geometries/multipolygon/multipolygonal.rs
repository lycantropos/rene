use crate::geometries::Polygon;
use crate::slice_sequence::SliceSequence;
use crate::traits::Multipolygonal;

use super::types::Multipolygon;

impl<'a, Scalar> Multipolygonal for &'a Multipolygon<Scalar> {
    type IndexPolygon = Polygon<Scalar>;
    type IntoIteratorPolygon = &'a Polygon<Scalar>;
    type Polygons = SliceSequence<'a, Polygon<Scalar>>;

    fn polygons(self) -> Self::Polygons {
        SliceSequence::new(&self.polygons)
    }
}

impl<Scalar> Multipolygonal for Multipolygon<Scalar> {
    type IndexPolygon = Polygon<Scalar>;
    type IntoIteratorPolygon = Polygon<Scalar>;
    type Polygons = Vec<Polygon<Scalar>>;

    fn polygons(self) -> Self::Polygons {
        self.polygons
    }
}
