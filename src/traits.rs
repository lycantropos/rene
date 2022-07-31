pub trait Point {
    type Coordinate;

    fn x(&self) -> Self::Coordinate;
    fn y(&self) -> Self::Coordinate;
}

pub trait Segment {
    type Point: self::Point;

    fn start(&self) -> Self::Point;
    fn end(&self) -> Self::Point;
}

pub trait Multisegment {
    type Point: self::Point;
    type Segment: self::Segment<Point = Self::Point>;

    fn segments(&self) -> Vec<Self::Segment>;
}

pub trait Contour {
    type Point: self::Point;
    type Segment: self::Segment<Point = Self::Point>;

    fn vertices(&self) -> Vec<Self::Point>;
    fn segments(&self) -> Vec<Self::Segment>;
}

pub trait Polygon {
    type Point: self::Point;
    type Segment: self::Segment<Point = Self::Point>;
    type Contour: self::Contour<Point = Self::Point, Segment = Self::Segment>;

    fn border(&self) -> Self::Contour;
    fn holes(&self) -> Vec<Self::Contour>;
}

pub trait Multipolygon {
    type Point: self::Point;
    type Segment: self::Segment<Point = Self::Point>;
    type Contour: self::Contour<Point = Self::Point, Segment = Self::Segment>;
    type Polygon: self::Polygon<
        Point = Self::Point,
        Segment = Self::Segment,
        Contour = Self::Contour,
    >;

    fn polygons(&self) -> Vec<Self::Polygon>;
}
