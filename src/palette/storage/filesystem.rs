use std::fs;
use std::path::{Path, PathBuf};

use super::{Palette, PaletteStorage, StorageError};

#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path:  impl AsRef<Path>) -> Result<Self, StorageError> {
        let base_path = base_path.as_ref().to_path_buf();

        fs::create_dir_all(&base_path)?;

        Ok(Self { base_path })
    }

    pub fn default_path() -> Result<PathBuf, StorageError> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            StorageError::PermissionDenied("Could not determine config directory".to_string())
        })?;

        Ok(config_dir.join("shinobu").join("palettes"))
    }

    fn palette_path(&self, name: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", name))
    }

    // TODO - does atomic write (temp file + rename) actually benefit?
    fn write(&self, path: &PathBuf, data: &str) -> Result<(), StorageError> {
        fs::write(path, data)?;
        Ok(())
}

impl Default for FileSystemStorage {
    fn default() -> Self {
        Self::new(Self::default_path().expect("Failed to get default path"))
            .expect("Failed to initialize FileSystemStorage with default path")
    }
}

impl PaletteStorage for FileSystemStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        let path = self.palette_path(&palette.name);

        let data = serde_json::to_string_pretty(palette)?;


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
