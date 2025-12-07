use std::fmt;

use crate::palette::Palette;

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
/// Each palette is stored individually, keyed by its name.
pub trait PaletteStorage {
    /// Save a palette. Uses `palette.name` as the storage key.
    fn save(&self, palette: &Palette) -> Result<(), StorageError>;

    /// Load a single palette by name.
    fn load(&self, name: &str) -> Result<Palette, StorageError>;

    /// Delete a palette by name.
    fn delete(&self, name: &str) -> Result<(), StorageError>;

    /// Check if a palette exists without loading it.
    fn exists(&self, name: &str) -> bool;

    /// List all stored palette names.
    /// Implementations should return just the names; the trait provides a default
    /// list() implementation that loads full palettes.
    fn list_names(&self) -> Result<Vec<String>, StorageError>;

    /// List all stored palettes.
    /// Default implementation loads each palette by name, skipping any that fail to load.
    /// Implementations can override this for more efficient bulk loading.
    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        let names = self.list_names()?;
        Ok(names
            .iter()
            .filter_map(|name| self.load(name).ok())
            .collect())
    }
}
