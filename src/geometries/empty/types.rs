#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Empty();

impl Empty {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
