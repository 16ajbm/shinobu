//! Storage implementation that saves palettes to the local filesystem.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Palette, PaletteStorage, StorageError};
use crate::palette::{PaletteId, PaletteName};

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
        let config_dir = dirs::config_dir().ok_or(StorageError::PermissionDenied)?;

        Ok(config_dir.join("shinobu").join("palettes"))
    }

    fn palette_path(&self, id: &PaletteId) -> PathBuf {
        self.base_path.join(format!("{}.json", id))
    }

    fn write(&self, path: &PathBuf, data: &str) -> Result<(), StorageError> {
        Ok(fs::write(path, data)?)
    }
}

impl Default for FileSystemStorage {
    fn default() -> Self {
        if let Ok(path) = Self::default_path() {
            if let Ok(storage) = Self::new(path) {
                return storage;
            }
        }

        // Should we propogate errors for the default storage path instead of silently falling back to a temp directory?
        let fallback_path = std::env::temp_dir().join("shinobu").join("palettes");

        let _ = fs::create_dir_all(&fallback_path);

        FileSystemStorage {
            base_path: fallback_path,
        }
    }
}

impl PaletteStorage for FileSystemStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        let path = self.palette_path(&palette.id);

        let data = serde_json::to_string_pretty(palette)?;

        self.write(&path, &data)
    }

    fn delete(&self, id: &PaletteId) -> Result<(), StorageError> {
        if !self.exists(id) {
            return Err(StorageError::NotFound(id.to_string()));
        }

        Ok(fs::remove_file(self.palette_path(id))?)
    }

    fn exists(&self, id: &PaletteId) -> bool {
        self.palette_path(id).exists()
    }

    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError> {
        Ok(self
            .load_all()?
            .iter()
            .map(|palette| palette.display_name.clone())
            .collect())
    }

    fn load(&self, id: &PaletteId) -> Result<Palette, StorageError> {
        let path = self.palette_path(id);

        let json = fs::read_to_string(path)?;

        Ok(serde_json::from_str(&json)?)
    }
    fn load_all(&self) -> Result<Vec<Palette>, StorageError> {
        // Read all JSON files in the base directory, skipping any that fail to load.
        let palettes: Vec<Palette> = fs::read_dir(&self.base_path)?
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let path = entry.path();

                if path.extension()?.to_str()? == "json" {
                    path.file_stem()?.to_str().map(|id| id.to_string())
                } else {
                    None
                }
            })
            .filter_map(|id| self.load(&id).ok())
            .collect();

        Ok(palettes)
    }
}
