#![forbid(unsafe_code)]
//! Transport-independent channel discovery for RSP.

use rsp_core::{
    ApplicationId, ChannelName, ContentDigest, ProtocolVersion, ReleaseId, ReleaseSequence,
    ReleaseVersion, TargetTriple,
};
use serde::{Deserialize, Serialize};

/// Current desired release for one application/channel/target stream.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChannelState {
    pub protocol: ProtocolVersion,
    pub application: ApplicationId,
    pub channel: ChannelName,
    pub target: Option<TargetTriple>,
    pub release: ReleaseId,
    pub version: ReleaseVersion,
    pub sequence: ReleaseSequence,
    pub manifest: ContentDigest,
    /// A client below this sequence may be required to update before continuing.
    pub minimum_supported_sequence: Option<ReleaseSequence>,
}
