use crate::geometries::Polygon;
use crate::slice_sequence::SliceSequence;
use crate::traits::Multipolygonal2;

use super::types::Multipolygon;

impl<'a, Scalar> Multipolygonal2 for &'a Multipolygon<Scalar> {
    type IndexPolygon = Polygon<Scalar>;
    type IntoIteratorPolygon = &'a Polygon<Scalar>;
    type Polygons = SliceSequence<'a, Polygon<Scalar>>;

    fn polygons2(self) -> Self::Polygons {
        SliceSequence::new(&self.polygons)
    }
}

impl<Scalar> Multipolygonal2 for Multipolygon<Scalar> {
    type IndexPolygon = Polygon<Scalar>;
    type IntoIteratorPolygon = Polygon<Scalar>;
    type Polygons = Vec<Polygon<Scalar>>;

    fn polygons2(self) -> Self::Polygons {
        self.polygons
    }
}
