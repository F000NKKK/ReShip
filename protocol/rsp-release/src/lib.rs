#![forbid(unsafe_code)]
//! Immutable logical release metadata for RSP.

use rsp_core::{ApplicationId, ContentDigest, ProtocolVersion, ReleaseId, ReleaseVersion, TargetId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Immutable logical application release.
///
/// A descriptor is channel-independent. The same descriptor may be promoted
/// through test, staging, stable, or any other channel without changing its
/// digest or target manifests.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDescriptor {
    pub protocol: ProtocolVersion,
    pub application: ApplicationId,
    pub release: ReleaseId,
    pub version: ReleaseVersion,
    pub targets: BTreeMap<TargetId, ContentDigest>,
}

impl ReleaseDescriptor {
    /// Returns the immutable manifest digest for `target` when this release
    /// contains that target.
    pub fn manifest_for(&self, target: &TargetId) -> Option<&ContentDigest> {
        self.targets.get(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsp_core::RSP_V1;

    #[test]
    fn one_release_can_reference_multiple_target_manifests() {
        let digest = ContentDigest::sha256(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        )
        .unwrap();
        let mut targets = BTreeMap::new();
        targets.insert(TargetId("win-x64".into()), digest.clone());
        targets.insert(TargetId("linux-x64".into()), digest.clone());

        let descriptor = ReleaseDescriptor {
            protocol: RSP_V1,
            application: ApplicationId("desktop".into()),
            release: ReleaseId("release-42".into()),
            version: ReleaseVersion("8.12.0".into()),
            targets,
        };

        assert_eq!(
            descriptor.manifest_for(&TargetId("win-x64".into())),
            Some(&digest)
        );
    }
}
