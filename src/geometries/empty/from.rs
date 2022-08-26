use super::types::Empty;

impl From<()> for Empty {
    fn from(_: ()) -> Self {
        Self::new()
    }
}
