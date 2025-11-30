use std::fmt;

use crate::palette::PaletteEntry;

mod memory;

#[cfg(not(target_arch = "wasm32"))]
mod filesystem;

#[cfg(target_arch = "wasm32")]
mod local;

pub use memory::InMemoryStorage;

#[cfg(not(target_arch = "wasm32"))]
pub use filesystem::FileSystemStorage;

#[cfg(target_arch = "wasm32")]
pub use local::LocalStorageAdapter;

/// Storage errors following Rust conventions.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    /// Entry with given ID not found.
    NotFound(String),
    /// Failed to serialize/deserialize data.
    SerializationError(String),
    /// Storage quota exceeded (e.g., LocalStorage limit, disk full).
    QuotaExceeded,
    /// Permission denied (e.g., localStorage disabled, filesystem access).
    PermissionDenied,
    /// I/O error (filesystem operations).
    IoError(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(id) => write!(f, "Palette not found: {}", id),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::QuotaExceeded => write!(f, "Storage quota exceeded"),
            Self::PermissionDenied => write!(f, "Storage permission denied"),
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

/// Storage abstraction for palette persistence.
/// Each palette is stored individually, keyed by its ID.
pub trait PaletteStorage {
    /// Save a palette entry. Uses `entry.id` as the storage key.
    fn save(&self, entry: &PaletteEntry) -> Result<(), StorageError>;

    /// Load a single palette entry by ID.
    fn load(&self, id: &str) -> Result<PaletteEntry, StorageError>;

    /// Delete a palette entry by ID.
    fn delete(&self, id: &str) -> Result<(), StorageError>;

    /// Check if a palette exists without loading it.
    fn exists(&self, id: &str) -> bool;

    /// List all stored palette IDs.
    /// Implementations should return just the IDs; the trait provides a default
    /// list() implementation that loads full entries.
    fn list_ids(&self) -> Result<Vec<String>, StorageError>;

    /// List all stored palette entries.
    /// Default implementation loads each entry by ID, skipping any that fail to load.
    /// Implementations can override this for more efficient bulk loading.
    fn list(&self) -> Result<Vec<PaletteEntry>, StorageError> {
        let ids = self.list_ids()?;
        Ok(ids.iter().filter_map(|id| self.load(id).ok()).collect())
    }
}
