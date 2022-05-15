use std::fmt;

use super::types::Point;

impl<Scalar: fmt::Display> fmt::Display for Point<Scalar> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_fmt(format_args!("Point({}, {})", self.0, self.1))
    }
}
