//! Storage implementation that saves palettes to the local filesystem.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Palette, PaletteStorage, StorageError};
use crate::palette::PaletteName;

#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let base_path = base_path.as_ref().to_path_buf();

        fs::create_dir_all(&base_path)?;

        Ok(Self { base_path })
    }

    pub fn default_path() -> Result<PathBuf, StorageError> {
        let config_dir = dirs::config_dir().ok_or_else(|| StorageError::PermissionDenied)?;

        Ok(config_dir.join("shinobu").join("palettes"))
    }

    fn palette_path(&self, name: &PaletteName) -> PathBuf {
        self.base_path.join(format!("{}.json", name))
    }

    // TODO - does atomic write (temp file + rename) actually benefit?
    fn write(&self, path: &PathBuf, data: &str) -> Result<(), StorageError> {
        Ok(fs::write(path, data)?)
    }
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

        self.write(&path, &data)
    }

    fn delete(&self, name: &PaletteName) -> Result<(), StorageError> {
        let path = self.palette_path(name);

        Ok(fs::remove_file(path)?)
    }

    fn exists(&self, name: &PaletteName) -> bool {
        self.palette_path(name).exists()
    }

    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        todo!()
    }

    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError> {
        let entries = fs::read_dir(&self.base_path)?;

        Ok(entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.extension()?.to_str()? == "json" {
                    Some(path.file_stem()?.to_str()?.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<PaletteName>>())
    }

    fn load(&self, name: &PaletteName) -> Result<Palette, StorageError> {
        let path = self.palette_path(name);

        let json = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&json)?)
    }
}
