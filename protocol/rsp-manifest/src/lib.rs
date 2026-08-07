#![forbid(unsafe_code)]
//! Immutable installation-snapshot model for RSP.

use rsp_core::{ApplicationId, ContentDigest, ProtocolVersion, ReleaseId, ReleaseSequence, ReleaseVersion, TargetTriple};
use serde::{Deserialize, Serialize};

/// Complete file snapshot for one immutable release.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub protocol: ProtocolVersion,
    pub application: ApplicationId,
    pub release: ReleaseId,
    pub version: ReleaseVersion,
    pub sequence: ReleaseSequence,
    pub target: Option<TargetTriple>,
    pub files: Vec<ManifestFile>,
}

/// One regular file in the target installation tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManifestFile {
    /// Normalized forward-slash relative path. Absolute paths are forbidden by the publisher.
    pub path: String,
    pub content: ContentDigest,
    pub size: u64,
    /// Portable executable intent for Unix-like targets. Windows ignores this flag.
    #[serde(default)]
    pub executable: bool,
}
