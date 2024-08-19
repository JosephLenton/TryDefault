use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

/// Error returned if there is no default value.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DefaultNotFound;

impl Error for DefaultNotFound {}

impl Display for DefaultNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "The type does not implement default")
    }
}
