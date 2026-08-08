#![forbid(unsafe_code)]
//! Transport-independent channel discovery for RSP.

use rsp_core::{ApplicationId, ChannelName, ChannelRevision, ContentDigest, ProtocolVersion};
use serde::{Deserialize, Serialize};

/// Current desired immutable release descriptor for one application channel.
///
/// `revision` orders changes to the channel pointer. Both forward promotion and
/// rollback increment it, so clients can distinguish a deliberate rollback from
/// stale discovery data without imposing an ordering on immutable releases.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChannelState {
    pub protocol: ProtocolVersion,
    pub application: ApplicationId,
    pub channel: ChannelName,
    pub revision: ChannelRevision,
    pub release: ContentDigest,
}

impl ChannelState {
    /// Returns true when this state is newer than a previously observed state
    /// for the same application/channel identity.
    pub fn is_newer_than(&self, previous: &Self) -> bool {
        self.application == previous.application
            && self.channel == previous.channel
            && self.revision > previous.revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsp_core::RSP_V1;

    fn state(revision: u64, digest: &str) -> ChannelState {
        ChannelState {
            protocol: RSP_V1,
            application: ApplicationId("desktop".into()),
            channel: ChannelName("stable".into()),
            revision: ChannelRevision(revision),
            release: ContentDigest::sha256(digest).unwrap(),
        }
    }

    #[test]
    fn rollback_still_advances_channel_revision() {
        let release_a = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let release_b = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

        let first = state(41, release_a);
        let promoted = state(42, release_b);
        let rolled_back = state(43, release_a);

        assert!(promoted.is_newer_than(&first));
        assert!(rolled_back.is_newer_than(&promoted));
        assert_eq!(rolled_back.release, first.release);
    }
}
