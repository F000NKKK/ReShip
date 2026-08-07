#![forbid(unsafe_code)]
//! HTTP binding for RSP. Semantic RSP crates do not depend on this crate.

use rsp_discovery::ChannelState;
use serde::{Deserialize, Serialize};

pub const CHANNEL_STATE_MEDIA_TYPE: &str = "application/vnd.reship.rsp.channel-state+json;v=1";
pub const MANIFEST_MEDIA_TYPE: &str = "application/vnd.reship.rsp.manifest+json;v=1";
pub const ARTIFACT_MEDIA_TYPE: &str = "application/octet-stream";

/// HTTP delivery metadata kept outside the transport-neutral `ChannelState`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChannelStateResponse {
    pub state: ChannelState,
    /// Ordered delivery bases. The first may be a CDN edge; later entries are fallbacks.
    pub delivery: Vec<DeliveryEndpoint>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeliveryEndpoint {
    pub base_url: String,
    pub region: Option<String>,
    pub priority: u16,
}

/// Immutable resources use content-addressed paths so a CDN can cache them indefinitely.
pub const MANIFEST_PATH_TEMPLATE: &str = "/rsp/v1/manifests/{algorithm}/{digest}";
pub const ARTIFACT_PATH_TEMPLATE: &str = "/rsp/v1/artifacts/{algorithm}/{digest}";
/// Channel state is mutable, small, and intended for ETag/conditional polling.
pub const CHANNEL_STATE_PATH_TEMPLATE: &str = "/rsp/v1/apps/{app}/channels/{channel}/state";
