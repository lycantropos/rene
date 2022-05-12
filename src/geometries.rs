use super::traits;
use std::fmt;

#[derive(Clone, fmt::Debug)]
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

impl<Scalar: fmt::Display> fmt::Display for Point<Scalar> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("Point({}, {})", self.0, self.1))
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

impl<Scalar: Eq> Eq for Point<Scalar> {}

#[derive(Clone)]
pub struct Segment<Scalar>(Point<Scalar>, Point<Scalar>);

impl<Scalar> Segment<Scalar> {
    pub fn new(start: Point<Scalar>, end: Point<Scalar>) -> Self {
        Self(start, end)
    }
}

impl<Scalar: Clone> traits::Segment<Scalar> for Segment<Scalar> {
    type Point = self::Point<Scalar>;

    fn start(&self) -> Self::Point {
        self.0.clone()
    }

    fn end(&self) -> Self::Point {
        self.1.clone()
    }
}

impl<Scalar: PartialEq> PartialEq for Segment<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
    }

    fn ne(&self, other: &Self) -> bool {
        (self.0 != other.0 && self.1 != other.0) || (self.0 != other.1 && self.1 != other.1)
    }
}

impl<Scalar: Eq> Eq for Segment<Scalar> {}

#[derive(Clone)]
pub struct Contour<Scalar>(Vec<Point<Scalar>>);

impl<Scalar: Clone> Contour<Scalar> {
    pub fn new(vertices: Vec<Point<Scalar>>) -> Self {
        Self(vertices)
    }
}

impl<Scalar: PartialEq> PartialEq for Contour<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        are_non_empty_unique_sequences_rotationally_equivalent(&self.0, &other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        !are_non_empty_unique_sequences_rotationally_equivalent(&self.0, &other.0)
    }
}

fn are_non_empty_unique_sequences_rotationally_equivalent<T: PartialEq>(
    left: &[T],
    right: &[T],
) -> bool {
    debug_assert!(!left.is_empty() && !right.is_empty());
    if left.len() != right.len() {
        false
    } else {
        let left_first_element = &left[0];
        right
            .iter()
            .position(|value| value == left_first_element)
            .map(|index| {
                (left[1..left.len() - index] == right[index + 1..]
                    && left[left.len() - index..] == right[..index])
                    || (left[left.len() - index..].iter().rev().eq(right[..index].iter())
                        && left[1..left.len() - index].iter().rev().eq(right[index + 1..].iter()))
            })
            .unwrap_or(false)
    }
}

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
