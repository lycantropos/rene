pub trait Point<Scalar> {
    fn x(&self) -> Scalar;
    fn y(&self) -> Scalar;
}

pub trait Segment<Scalar> {
    type Point: Point<Scalar>;

    fn start(&self) -> Self::Point;
    fn end(&self) -> Self::Point;
}

pub trait Multisegment<Scalar> {
    type Point: self::Point<Scalar>;
    type Segment: self::Segment<Scalar, Point = Self::Point>;

    fn segments(&self) -> Vec<Self::Segment>;
}

pub trait Contour<Scalar> {
    type Point: self::Point<Scalar>;
    type Segment: self::Segment<Scalar, Point = Self::Point>;

    fn vertices(&self) -> Vec<Self::Point>;
    fn segments(&self) -> Vec<Self::Segment>;
}

pub trait Polygon<Scalar> {
    type Point: self::Point<Scalar>;
    type Segment: self::Segment<Scalar, Point = Self::Point>;
    type Contour: self::Contour<Scalar, Point = Self::Point, Segment = Self::Segment>;

    fn border(&self) -> Self::Contour;
    fn holes(&self) -> Vec<Self::Contour>;
}

pub trait Multipolygon<Scalar> {
    type Point: self::Point<Scalar>;
    type Segment: self::Segment<Scalar, Point = Self::Point>;
    type Contour: self::Contour<Scalar, Point = Self::Point, Segment = Self::Segment>;
    type Polygon: self::Polygon<
        Scalar,
        Point = Self::Point,
        Segment = Self::Segment,
        Contour = Self::Contour,
    >;

    fn polygons(&self) -> Vec<Self::Polygon>;
}
