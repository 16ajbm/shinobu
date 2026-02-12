use super::Palette;

mod memory;
mod filesystem;
mod local;

trait PaletteStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError>;
    fn load(&self, name: &str) -> Result<Palette, StorageError>;
    fn delete(&self, name: &str) -> Result<(), StorageError>;
    fn exists(&self, name: &str) -> bool;
    fn list_names(&self) -> Vec<String>;
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
