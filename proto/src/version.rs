use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A request to negotiate the version of the report format to use.
/// This is used to ensure that the client and server are speaking the same language.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VersionNegotiationRequest {
    /// The maximum version that the client supports.
    /// The client is assumed to support _all_ versions before this one.
    pub max_supported: u32,
}

/// The response to a version negotiation request.
/// This contains the version that the server has chosen for the client to use.
/// The client should be able to handle this version, as they indicated that they support it in the request.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VersionNegotiationResponse {
    /// Version is the version of the Report protocol to use.
    /// [u32::MAX] is reserved for later use.
    #[schemars(range(max = 4294967294u32))]
    pub version: u32,
}
