use super::traits;

#[derive(Clone)]
pub struct Point<Scalar>(Scalar, Scalar);

impl<Scalar> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self(x, y)
    }
}

impl<Scalar: Clone> traits::Point<Scalar> for Point<Scalar> {
    fn x(&self) -> Scalar {
        self.0.clone()
    }

    fn y(&self) -> Scalar {
        self.1.clone()
    }
}

impl<Scalar: PartialEq> PartialEq for Point<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}

#[derive(Clone)]
struct Segment<Scalar>(Point<Scalar>, Point<Scalar>);

impl<Scalar: Clone> traits::Segment<Scalar> for Segment<Scalar> {
    type Point = self::Point<Scalar>;

    fn start(&self) -> Self::Point {
        self.0.clone()
    }

    fn end(&self) -> Self::Point {
        self.1.clone()
    }
}

#[derive(Clone)]
struct Contour<Scalar>(Vec<Point<Scalar>>);

impl<Scalar: Clone> traits::Contour<Scalar> for Contour<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;

    fn vertices(&self) -> Vec<Self::Point> {
        self.0.clone()
    }

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.0.len());
        for index in 0..self.0.len() - 1 {
            result.push(Segment(self.0[index].clone(), self.0[index + 1].clone()))
        }
        result.push(Segment(self.0[self.0.len() - 1].clone(), self.0[0].clone()));
        result
    }
}

#[derive(Clone)]
struct Polygon<Scalar>(Contour<Scalar>, Vec<Contour<Scalar>>);

impl<Scalar: Clone> traits::Polygon<Scalar> for Polygon<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;
    type Contour = self::Contour<Scalar>;

    fn border(&self) -> Self::Contour {
        self.0.clone()
    }

    fn holes(&self) -> Vec<Self::Contour> {
        self.1.clone()
    }
}

#[derive(Clone)]
struct Multipolygon<Scalar>(Vec<Polygon<Scalar>>);

impl<Scalar: Clone> traits::Multipolygon<Scalar> for Multipolygon<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;
    type Contour = self::Contour<Scalar>;
    type Polygon = self::Polygon<Scalar>;

    fn polygons(&self) -> Vec<Self::Polygon> {
        self.0.clone()
    }
}
