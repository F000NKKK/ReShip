#![forbid(unsafe_code)]
//! Immutable installation-snapshot model for RSP.

use core::fmt;
use rsp_core::{ContentDigest, ContentLength, ProtocolVersion, TargetId};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::collections::BTreeMap;

/// Complete immutable filesystem snapshot for one release target.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub protocol: ProtocolVersion,
    pub target: TargetId,
    pub files: BTreeMap<ManifestPath, ManifestFile>,
}

/// One regular file in the target installation tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ManifestFile {
    pub content: ContentDigest,
    pub size: ContentLength,
    /// Portable executable intent for Unix-like targets. Windows ignores this flag.
    #[serde(default)]
    pub executable: bool,
}

/// Canonical portable relative path inside a managed installation snapshot.
///
/// RSP uses one conservative path grammar for every target so a manifest cannot
/// acquire platform-specific absolute-path, alternate-stream, or reserved-name
/// semantics when materialized. Native materializers must additionally reject
/// distinct manifest paths that collide under the destination filesystem's own
/// case-folding or Unicode-normalization rules.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ManifestPath(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidManifestPath;

impl fmt::Display for InvalidManifestPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid canonical manifest path")
    }
}

impl std::error::Error for InvalidManifestPath {}

impl ManifestPath {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidManifestPath> {
        let value = value.into();
        validate_path(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_path(value: &str) -> Result<(), InvalidManifestPath> {
    if value.is_empty()
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains('\\')
        || value.contains('\0')
        || value.split('/').any(invalid_segment)
    {
        Err(InvalidManifestPath)
    } else {
        Ok(())
    }
}

fn invalid_segment(segment: &str) -> bool {
    if segment.is_empty()
        || segment == "."
        || segment == ".."
        || segment.ends_with([' ', '.'])
        || segment.chars().any(|character| {
            character.is_control() || matches!(character, '<' | '>' | ':' | '"' | '|' | '?' | '*')
        })
    {
        return true;
    }

    let stem = segment.split('.').next().unwrap_or(segment);
    is_windows_reserved_name(stem)
}

fn is_windows_reserved_name(stem: &str) -> bool {
    let name = stem.to_ascii_uppercase();
    matches!(name.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || matches!(
            name.as_str(),
            "COM1" | "COM2" | "COM3" | "COM4" | "COM5" | "COM6" | "COM7" | "COM8" | "COM9"
        )
        || matches!(
            name.as_str(),
            "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7" | "LPT8" | "LPT9"
        )
}

impl fmt::Display for ManifestPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for ManifestPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ManifestPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::new(String::deserialize(deserializer)?).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_path_rejects_non_canonical_paths() {
        for value in [
            "",
            "/app.exe",
            "a/",
            "a//b",
            "a/./b",
            "a/../b",
            "a\\b",
            "C:/app.exe",
            "bin/file:stream",
            "bin/file?.dll",
            "bin/file*",
            "bin/trailing. ",
            "bin/trailing.",
            "bin/CON",
            "bin/con.txt",
            "bin/COM1.dll",
            "bin/LPT9",
            "bin/control\u{1f}.txt",
        ] {
            assert!(ManifestPath::new(value).is_err(), "accepted {value:?}");
        }
    }

    #[test]
    fn manifest_path_accepts_portable_relative_paths() {
        for value in [
            "bin/sub/app.exe",
            "assets/data-1.json",
            "dir.name/file_name",
        ] {
            let path = ManifestPath::new(value).unwrap();
            assert_eq!(path.as_str(), value);
        }
    }
}
