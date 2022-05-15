use crate::geometries::{Contour, Point, Polygon, Segment};
use crate::traits;

#[derive(Clone)]
struct Multipolygon<Scalar> {
    polygons: Vec<Polygon<Scalar>>,
}

impl<Scalar: Clone> traits::Multipolygon<Scalar> for Multipolygon<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;
    type Contour = self::Contour<Scalar>;
    type Polygon = self::Polygon<Scalar>;

    fn polygons(&self) -> Vec<Self::Polygon> {
        self.polygons.clone()
    }
}
