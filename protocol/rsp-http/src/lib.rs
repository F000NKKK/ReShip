#![forbid(unsafe_code)]
//! HTTP binding for RSP. Semantic RSP crates do not depend on this crate.

use serde::{Deserialize, Serialize};

pub const CHANNEL_STATE_MEDIA_TYPE: &str = "application/vnd.reship.rsp.channel-state+json;v=1";
pub const RELEASE_MEDIA_TYPE: &str = "application/vnd.reship.rsp.release+json;v=1";
pub const MANIFEST_MEDIA_TYPE: &str = "application/vnd.reship.rsp.manifest+json;v=1";
pub const DELIVERY_MEDIA_TYPE: &str = "application/vnd.reship.rsp.delivery+json;v=1";
pub const ARTIFACT_MEDIA_TYPE: &str = "application/octet-stream";

/// One ordered HTTP base used to retrieve immutable RSP resources.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeliveryEndpoint {
    pub base_url: String,
    pub region: Option<String>,
    pub priority: u16,
}

/// Delivery topology is intentionally separate from mutable channel state.
///
/// A client may move regions or an edge may become unavailable without any
/// application release or channel mutation occurring.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeliveryResponse {
    pub endpoints: Vec<DeliveryEndpoint>,
}

/// Mutable desired-state resource. Its representation should be conditionally
/// revalidated with ETag rather than cached as immutable content.
pub const CHANNEL_STATE_PATH_TEMPLATE: &str = "/rsp/v1/apps/{app}/channels/{channel}/state";

/// Region/topology-specific delivery discovery, independent of channel state.
pub const DELIVERY_PATH_TEMPLATE: &str = "/rsp/v1/apps/{app}/delivery";

/// Immutable resources use digest-addressed paths and may be cached with a long
/// freshness lifetime because a digest URL never changes its representation.
pub const RELEASE_PATH_TEMPLATE: &str = "/rsp/v1/releases/{algorithm}/{digest}";
pub const MANIFEST_PATH_TEMPLATE: &str = "/rsp/v1/manifests/{algorithm}/{digest}";
pub const ARTIFACT_PATH_TEMPLATE: &str = "/rsp/v1/artifacts/{algorithm}/{digest}";

/// Recommended Cache-Control value for immutable descriptor/manifest/artifact resources.
pub const IMMUTABLE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable, no-transform";

/// Recommended Cache-Control value for channel state: caches may retain the
/// representation but must revalidate it before reuse.
pub const CHANNEL_STATE_CACHE_CONTROL: &str = "no-cache";
