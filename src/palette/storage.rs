use super::Palette;
use crate::palette::PaletteName;

#[cfg(not(target_arch = "wasm32"))]
mod filesystem;
#[cfg(target_arch = "wasm32")]
mod local;
#[cfg(not(target_arch = "wasm32"))]
mod memory;

/// Storage abstraction for `Palette` persistence. Each `Palette` is stored individually, keyed by it's `name`.
pub trait PaletteStorage {
    /// Saves a `Palette`.
    fn save(&self, palette: &Palette) -> Result<(), StorageError>;
    /// Loads a single `Palette` by name.
    fn load(&self, name: &PaletteName) -> Result<Palette, StorageError>;
    /// Deletes a single `Palette` by name.
    fn delete(&self, name: &PaletteName) -> Result<(), StorageError>;
    /// Checks if a `Palette` exists.
    fn exists(&self, name: &PaletteName) -> bool;
    /// Lists the `name`s of all stored `Palettes`.
    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError>;
    /// Lists all stored `Palettes`. Defaults to directly `load`-ing each `Palette` by `name`.
    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        Ok(self
            .list_names()?
            .iter()
            .filter_map(|name| self.load(name).ok())
            .collect())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Palette not found: {0}")]
    NotFound(String),
    #[error("Unknown error: {0}")]
    Error(String),
}
