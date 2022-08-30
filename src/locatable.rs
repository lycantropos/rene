#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Location {
    /// point lies on the boundary of the geometry
    Boundary,
    /// point lies in the exterior of the geometry
    Exterior,
    /// point lies in the interior of the geometry
    Interior,
}
