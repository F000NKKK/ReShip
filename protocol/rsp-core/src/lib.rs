#![forbid(unsafe_code)]
//! Transport-independent primitives for the ReShip Protocol (RSP).

use core::{fmt, str::FromStr};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
    TargetId,
    "Opaque release target selector such as win-x64 or linux-arm64."
);

/// Monotonic mutation revision of one application/channel pointer.
///
/// A promotion and a rollback both increment this value. It therefore orders
/// channel-state mutations rather than application releases.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChannelRevision(pub u64);

/// Byte length encoded as a decimal string on the wire.
///
/// Encoding 64-bit values as strings avoids precision loss in JSON consumers
/// whose native number type is IEEE-754 binary64.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContentLength(pub u64);

macro_rules! decimal_u64_wire {
    ($name:ident) => {
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0.to_string())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                if value.is_empty()
                    || (value.len() > 1 && value.starts_with('0'))
                    || !value.bytes().all(|byte| byte.is_ascii_digit())
                {
                    return Err(de::Error::custom("expected canonical unsigned decimal string"));
                }

                value
                    .parse::<u64>()
                    .map(Self)
                    .map_err(de::Error::custom)
            }
        }
    };
}

decimal_u64_wire!(ChannelRevision);
decimal_u64_wire!(ContentLength);

/// Hash algorithm used by a content-addressed resource.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DigestAlgorithm {
    Sha256,
}

impl DigestAlgorithm {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
        }
    }
}

/// Error returned when parsing a content digest.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseContentDigestError;

impl fmt::Display for ParseContentDigestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid canonical content digest")
    }
}

impl std::error::Error for ParseContentDigestError {}

/// Digest identifying immutable manifests, release descriptors, and artifacts.
///
/// RSP v1 serializes SHA-256 digests as `sha256:` followed by exactly 64
/// lowercase hexadecimal characters.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContentDigest {
    algorithm: DigestAlgorithm,
    value: String,
}

impl ContentDigest {
    /// Creates a validated SHA-256 digest from 64 lowercase hexadecimal digits.
    pub fn sha256(value: impl Into<String>) -> Result<Self, ParseContentDigestError> {
        let value = value.into();
        validate_sha256(&value)?;
        Ok(Self {
            algorithm: DigestAlgorithm::Sha256,
            value,
        })
    }

    pub const fn algorithm(&self) -> DigestAlgorithm {
        self.algorithm
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn validate_sha256(value: &str) -> Result<(), ParseContentDigestError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(ParseContentDigestError)
    }
}

impl fmt::Display for ContentDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.algorithm.as_str(), self.value)
    }
}

impl FromStr for ContentDigest {
    type Err = ParseContentDigestError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digest = value
            .strip_prefix("sha256:")
            .ok_or(ParseContentDigestError)?;
        Self::sha256(digest)
    }
}

impl Serialize for ContentDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for ContentDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_revision_uses_decimal_string_wire_format() {
        let encoded = serde_json::to_string(&ChannelRevision(u64::MAX)).unwrap();
        assert_eq!(encoded, format!("\"{}\"", u64::MAX));
        assert_eq!(
            serde_json::from_str::<ChannelRevision>(&encoded).unwrap(),
            ChannelRevision(u64::MAX)
        );
    }

    #[test]
    fn decimal_wire_rejects_non_canonical_forms() {
        assert!(serde_json::from_str::<ChannelRevision>("\"01\"").is_err());
        assert!(serde_json::from_str::<ChannelRevision>("1").is_err());
        assert!(serde_json::from_str::<ContentLength>("\"+1\"").is_err());
    }

    #[test]
    fn digest_wire_format_is_canonical_and_validated() {
        let value = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let digest = ContentDigest::sha256(value).unwrap();
        assert_eq!(
            serde_json::to_string(&digest).unwrap(),
            format!("\"sha256:{value}\"")
        );
        assert!("sha256:ABCDEF".parse::<ContentDigest>().is_err());
        assert!("sha512:00".parse::<ContentDigest>().is_err());
    }
}
