use super::Palette;
use crate::palette::{PaletteId, PaletteName};

#[cfg(not(target_arch = "wasm32"))]
mod filesystem;
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
mod in_memory;
#[cfg(any(target_arch = "wasm32", test))]
mod local_storage;

/// Storage abstraction for `Palette` persistence. Each `Palette` is stored individually, keyed by it's `PaletteId`.
pub trait PaletteStorage {
    /// Saves a `Palette`.
    fn save(&self, palette: &Palette) -> Result<(), StorageError>;
    /// Loads a single `Palette` by name.
    fn load(&self, id: &PaletteId) -> Result<Palette, StorageError>;
    fn load_all(&self) -> Result<Vec<Palette>, StorageError>;
    /// Deletes a single `Palette` by name.
    fn delete(&self, id: &PaletteId) -> Result<(), StorageError>;
    /// Checks if a `Palette` exists.
    fn exists(&self, id: &PaletteId) -> bool;
    /// Lists the `name`s of all stored `Palettes`.
    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError>;
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
    #[allow(dead_code)]
    Error(String),
}

pub fn save(palette: &Palette) -> Result<(), StorageError> {
    get_storage().save(palette)
}

pub fn load_all() -> Result<Vec<Palette>, StorageError> {
    get_storage().load_all()
}

#[allow(dead_code)]

pub fn load(id: &PaletteId) -> Result<Palette, StorageError> {
    get_storage().load(id)
}

pub fn delete(id: &PaletteId) -> Result<(), StorageError> {
    get_storage().delete(id)
}

#[allow(dead_code)]
pub fn exists(id: &PaletteId) -> bool {
    get_storage().exists(id)
}

#[allow(dead_code)]

pub fn list_names() -> Result<Vec<PaletteName>, StorageError> {
    get_storage().list_names()
}

fn get_storage() -> Box<dyn PaletteStorage> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(filesystem::FileSystemStorage::default())
    }
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(local_storage::LocalStorage::default())
    }
}
