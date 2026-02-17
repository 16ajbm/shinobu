use super::Palette;
use crate::palette::PaletteName;

mod filesystem;
mod local;
mod memory;

trait PaletteStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError>;
    fn load(&self, name: &PaletteName) -> Result<Palette, StorageError>;
    fn delete(&self, name: &PaletteName) -> Result<(), StorageError>;
    fn exists(&self, name: &PaletteName) -> bool;
    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError>;
    fn list(&self) -> Result<Vec<Palette>, StorageError>;
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Palette not found: {0}")]
    NotFound(String),
}
