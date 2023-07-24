use crate::geometries::Point;
use crate::traits::Segmental;

use super::types::Segment;

impl<'a, Scalar> Segmental for &'a Segment<Scalar> {
    type Endpoint = &'a Point<Scalar>;

    fn start(self) -> Self::Endpoint {
        &self.start
    }

    fn end(self) -> Self::Endpoint {
        &self.end
    }

    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint) {
        (&self.start, &self.end)
    }
}

impl<Scalar> Segmental for Segment<Scalar> {
    type Endpoint = Point<Scalar>;

    fn start(self) -> Self::Endpoint {
        self.start
    }

    fn end(self) -> Self::Endpoint {
        self.end
    }

    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint) {
        (self.start, self.end)
    }
}
