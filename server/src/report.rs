mod golang;
mod jacoco;

/// The input report was invalid.
/// There are no details as this should be regarded as an unrecoverable error; a new upload would be required.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("the report was formatted incorrectly")]
pub struct InvalidReport;
