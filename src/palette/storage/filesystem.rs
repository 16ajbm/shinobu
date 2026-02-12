use std::fs;
use std::path::{Path, PathBuf};

use super::{Palette, PaletteStorage, StorageError};

#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(path: Option<impl AsRef<Path>>) -> Result<Self, StorageError> {
        let base_path = match path {
            Some(p) => p.as_ref().to_path_buf(),
            None => Self::default_path()?,
        };

        fs::create_dir_all(&base_path)?;

        Ok(Self { base_path })
    }

    pub fn default_path() -> Result<PathBuf, StorageError> {
        let config_dir = dirs::config_dir().ok_or_else(|| StorageError::PermissionDenied("Could not determine config directory".to_string()))?;

        Ok(config_dir.join("shinobu").join("palettes"))
    }
}

impl Default for FileSystemStorage {
    fn default() -> Self {
        Self::new(None).expect("Failed to initialize FileSystemStorage with default path")
    }
}

impl PaletteStorage for FileSystemStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        todo!()
    }

    fn delete(&self, name: &str) -> Result<(), StorageError> {
        todo!()
    }

    fn exists(&self, name: &str) -> bool {
        todo!()
    }

    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        todo!()
    }

    fn list_names(&self) -> Vec<String> {
        todo!()
    }

    fn load(&self, name: &str) -> Result<Palette, StorageError> {
        todo!()
    }
}