#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OverlapKind {
    None,
    SameOrientation,
    DifferentOrientation,
}
