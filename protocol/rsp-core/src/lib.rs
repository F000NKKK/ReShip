#![forbid(unsafe_code)]
//! Transport-independent primitives for the ReShip Protocol (RSP).

use serde::{Deserialize, Serialize};

/// RSP semantic protocol version.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

/// Initial RSP protocol version.
pub const RSP_V1: ProtocolVersion = ProtocolVersion { major: 1, minor: 0 };

macro_rules! string_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);
    };
}

string_id!(ApplicationId, "Stable application identifier.");
string_id!(ChannelName, "Release channel name such as test or stable.");
string_id!(ReleaseId, "Opaque immutable release identifier.");
string_id!(ReleaseVersion, "Human-facing application version string.");
string_id!(
    TargetTriple,
    "Opaque target/platform selector owned by the publisher."
);

/// Monotonic release order within one application/channel/target stream.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReleaseSequence(pub u64);

/// Hash algorithm used by a content-addressed resource.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestAlgorithm {
    Sha256,
}

/// Digest identifying immutable manifests and artifacts.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ContentDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
}
