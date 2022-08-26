#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Empty();

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}
