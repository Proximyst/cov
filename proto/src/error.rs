use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An error response, accompanied with an error HTTP status code.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Error {
    /// The identifying error code. This can give more information than the HTTP status code.
    pub code: ErrorCode,

    /// A human-readable message that can be used for debugging.
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorCode {
    /// The request was unauthorized. This can entail lacking scope, or not having a valid authentication token.
    Unauthorized,

    /// The resource was not found. This can mean you do not have permission to see the resource, or that it simply is a wrong turn.
    NotFound,

    /// The request wanted a media type we cannot support. It must include `application/json` in the `Accept` header somewhere.
    /// Values like `application/*` and `*/*` are also accepted. Omitting the header entirely is also accepted.
    NotAcceptable,

    /// The request was invalid because of another resource. The message can contain more information.
    Precondition,

    /// The resource provided already exists.
    AlreadyExists,

    /// The client does not know about the error code.
    /// This is never returned by the server.
    #[serde(other)]
    Unknown,
}
