use std::cell::RefCell;
use std::collections::HashMap;

use super::{PaletteStorage, StorageError};
use crate::palette::Palette;

/// In-memory storage implementation using HashMap.
/// Data is not persisted - lost when the instance is dropped.
/// Thread-safe for single-threaded use (uses RefCell for interior mutability).
#[derive(Debug)]
pub struct InMemoryStorage {
    data: RefCell<HashMap<String, Palette>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }

    /// Create with pre-populated palettes (useful for testing).
    #[cfg(test)]
    pub fn with_palettes(palettes: Vec<Palette>) -> Self {
        let mut map = HashMap::new();
        for palette in palettes {
            map.insert(palette.name.clone(), palette);
        }
        Self {
            data: RefCell::new(map),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl PaletteStorage for InMemoryStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        self.data
            .borrow_mut()
            .insert(palette.name.clone(), palette.clone());
        Ok(())
    }

    fn load(&self, name: &str) -> Result<Palette, StorageError> {
        self.data
            .borrow()
            .get(name)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(name.to_string()))
    }

    fn delete(&self, name: &str) -> Result<(), StorageError> {
        self.data
            .borrow_mut()
            .remove(name)
            .ok_or_else(|| StorageError::NotFound(name.to_string()))?;
        Ok(())
    }

    fn exists(&self, name: &str) -> bool {
        self.data.borrow().contains_key(name)
    }

    fn list_names(&self) -> Result<Vec<String>, StorageError> {
        Ok(self.data.borrow().keys().cloned().collect())
    }

    // Override default implementation for efficiency - direct access to all values
    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        Ok(self.data.borrow().values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::Palette;

    fn create_test_palette(name: &str) -> Palette {
        Palette::custom(name.to_string(), Palette::red().colors)
    }

    #[test]
    fn test_save_and_load() {
        let storage = InMemoryStorage::new();
        let palette = create_test_palette("Test");

        storage.save(&palette).unwrap();
        let loaded = storage.load(&palette.name).unwrap();

        assert_eq!(loaded.name, palette.name);
        assert_eq!(loaded.colors, palette.colors);
    }

    #[test]
    fn test_delete_nonexistent_returns_error() {
        let storage = InMemoryStorage::new();

        assert!(storage.delete("nonexistent").is_err());
    }

    #[test]
    fn test_load_nonexistent_returns_error() {
        let storage = InMemoryStorage::new();

        assert!(storage.load("nonexistent").is_err());
    }

    #[test]
    fn test_save_overwrites() {
        let storage = InMemoryStorage::new();
        let palette = create_test_palette("Test");
        let mut updated = palette.clone();
        updated.colors[0] = "#ffffff".to_string();

        storage.save(&palette).unwrap();
        storage.save(&updated).unwrap();

        assert_eq!(storage.list().unwrap().len(), 1);
        assert_eq!(storage.load(&palette.name).unwrap().colors[0], "#ffffff");
    }

    #[test]
    fn test_list_empty_storage() {
        let storage = InMemoryStorage::new();
        assert_eq!(storage.list().unwrap().len(), 0);
    }
}
