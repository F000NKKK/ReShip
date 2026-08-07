#![forbid(unsafe_code)]
//! ReShip storage boundary. Storage implementations are outside RSP.

use rsp_core::ContentDigest;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("artifact is not present")]
    NotFound,
    #[error("storage backend failure: {0}")]
    Backend(String),
}

/// Origin-side artifact metadata. Actual streaming backends are added behind this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredArtifact {
    pub digest: ContentDigest,
    pub size: u64,
    pub local_path: Option<PathBuf>,
}
