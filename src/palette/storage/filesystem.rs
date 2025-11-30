use std::fs;
use std::path::PathBuf;

use super::{PaletteStorage, StorageError};
use crate::palette::PaletteEntry;

/// Filesystem-based storage using per-palette JSON files.
/// Stores palettes in: ~/.config/shinobu/palettes/<id>.json
/// Uses atomic writes (write to temp file, then rename) for safety.
#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    base_path: PathBuf,
}

impl FileSystemStorage {
    /// Create a new FileSystemStorage in the default location.
    /// Creates the directory if it doesn't exist.
    pub fn new() -> Result<Self, StorageError> {
        let base = Self::default_path()?;
        fs::create_dir_all(&base).map_err(|e| StorageError::IoError(e.to_string()))?;

        Ok(Self { base_path: base })
    }

    /// Create with a custom base path (useful for testing).
    pub fn with_path(path: PathBuf) -> Result<Self, StorageError> {
        fs::create_dir_all(&path).map_err(|e| StorageError::IoError(e.to_string()))?;
        Ok(Self { base_path: path })
    }

    /// Get the default storage path: ~/.config/shinobu/palettes
    ///
    /// Note: Uses `dirs` crate - isolated here for easy replacement if needed.
    /// Fallback implementation: `std::env::var("HOME")?.join(".config/shinobu/palettes")`
    fn default_path() -> Result<PathBuf, StorageError> {
        let config_dir = get_config_dir().ok_or(StorageError::PermissionDenied)?;
        Ok(config_dir.join("shinobu").join("palettes"))
    }

    /// Get the file path for a palette ID.
    fn palette_path(&self, id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", id))
    }

    /// Atomic write: write to temp file, then rename.
    fn atomic_write(&self, path: &PathBuf, content: &str) -> Result<(), StorageError> {
        let temp_path = path.with_extension("json.tmp");

        // Write to temp file
        fs::write(&temp_path, content).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                StorageError::PermissionDenied
            } else {
                StorageError::IoError(e.to_string())
            }
        })?;

        // Atomic rename
        fs::rename(&temp_path, path).map_err(|e| StorageError::IoError(e.to_string()))?;

        Ok(())
    }
}

impl Default for FileSystemStorage {
    fn default() -> Self {
        Self::new().expect("Failed to create default FileSystemStorage")
    }
}

impl PaletteStorage for FileSystemStorage {
    fn save(&self, entry: &PaletteEntry) -> Result<(), StorageError> {
        let path = self.palette_path(entry.id.as_str());

        let json = serde_json::to_string_pretty(entry)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        self.atomic_write(&path, &json)?;
        Ok(())
    }

    fn load(&self, id: &str) -> Result<PaletteEntry, StorageError> {
        let path = self.palette_path(id);

        let json = fs::read_to_string(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound(id.to_string())
            } else {
                StorageError::IoError(e.to_string())
            }
        })?;

        serde_json::from_str(&json).map_err(|e| StorageError::SerializationError(e.to_string()))
    }

    fn delete(&self, id: &str) -> Result<(), StorageError> {
        let path = self.palette_path(id);

        fs::remove_file(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound(id.to_string())
            } else {
                StorageError::IoError(e.to_string())
            }
        })?;

        Ok(())
    }

    fn exists(&self, id: &str) -> bool {
        self.palette_path(id).exists()
    }

    fn list_ids(&self) -> Result<Vec<String>, StorageError> {
        let dir_entries = fs::read_dir(&self.base_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                // Directory doesn't exist yet - return empty list
                return StorageError::NotFound("directory".to_string());
            }
            StorageError::IoError(e.to_string())
        });

        // If directory doesn't exist, return empty list
        let dir_entries = match dir_entries {
            Ok(entries) => entries,
            Err(StorageError::NotFound(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e),
        };

        let mut ids = Vec::new();
        for entry in dir_entries {
            let entry = entry.map_err(|e| StorageError::IoError(e.to_string()))?;
            let path = entry.path();

            // Only process .json files (skip .tmp files)
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                    ids.push(id.to_string());
                }
            }
        }

        Ok(ids)
    }
}

/// Get the platform-specific config directory.
///
/// Returns:
/// - macOS: ~/Library/Application Support
/// - Linux: ~/.config (or $XDG_CONFIG_HOME)
/// - Windows: %APPDATA%
fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::Palette;
    use std::env;

    /// RAII fixture for temporary test directories.
    /// Automatically cleans up on drop, even if test panics.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            Self(env::temp_dir().join(format!("shinobu_test_{}", rand::random::<u64>())))
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).ok();
        }
    }

    impl std::ops::Deref for TempDir {
        type Target = PathBuf;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn test_save_and_load_from_disk() {
        let temp = TempDir::new();
        let storage = FileSystemStorage::with_path(temp.clone()).unwrap();
        let entry = PaletteEntry::new_custom("Test".to_string(), Palette::red());

        storage.save(&entry).unwrap();
        let loaded = storage.load(entry.id.as_str()).unwrap();

        assert_eq!(loaded.name, "Test");
        assert!(temp.join(format!("{}.json", entry.id.as_str())).exists());
    }

    #[test]
    fn test_delete_removes_file() {
        let temp = TempDir::new();
        let storage = FileSystemStorage::with_path(temp.clone()).unwrap();
        let entry = PaletteEntry::new_custom("Test".to_string(), Palette::red());

        storage.save(&entry).unwrap();
        storage.delete(entry.id.as_str()).unwrap();

        assert!(!storage.exists(entry.id.as_str()));
    }

    #[test]
    fn test_list_multiple_files() {
        let temp = TempDir::new();
        let storage = FileSystemStorage::with_path(temp.clone()).unwrap();
        let entry1 = PaletteEntry::new_custom("First".to_string(), Palette::red());
        std::thread::sleep(std::time::Duration::from_millis(2));
        let entry2 = PaletteEntry::new_custom("Second".to_string(), Palette::green());

        storage.save(&entry1).unwrap();
        storage.save(&entry2).unwrap();

        assert_eq!(storage.list().unwrap().len(), 2);
    }

    #[test]
    fn test_atomic_write_no_temp_files() {
        let temp = TempDir::new();
        let storage = FileSystemStorage::with_path(temp.clone()).unwrap();
        let entry = PaletteEntry::new_custom("Test".to_string(), Palette::red());

        storage.save(&entry).unwrap();

        // Verify no .tmp files left behind
        for file in fs::read_dir(temp.deref()).unwrap() {
            let path = file.unwrap().path();
            assert_ne!(path.extension().and_then(|s| s.to_str()), Some("tmp"));
        }
    }
}
