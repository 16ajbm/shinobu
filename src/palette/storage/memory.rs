use std::cell::RefCell;
use std::collections::HashMap;

use super::{PaletteStorage, StorageError};
use crate::palette::PaletteEntry;

/// In-memory storage implementation using HashMap.
/// Data is not persisted - lost when the instance is dropped.
/// Thread-safe for single-threaded use (uses RefCell for interior mutability).
#[derive(Debug)]
pub struct InMemoryStorage {
    data: RefCell<HashMap<String, PaletteEntry>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }

    /// Create with pre-populated entries (useful for testing).
    #[cfg(test)]
    pub fn with_entries(entries: Vec<PaletteEntry>) -> Self {
        let mut map = HashMap::new();
        for entry in entries {
            map.insert(entry.id.as_str().to_string(), entry);
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
    fn save(&self, entry: &PaletteEntry) -> Result<(), StorageError> {
        let key = entry.id.as_str().to_string();
        self.data.borrow_mut().insert(key, entry.clone());
        Ok(())
    }

    fn load(&self, id: &str) -> Result<PaletteEntry, StorageError> {
        self.data
            .borrow()
            .get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(id.to_string()))
    }

    fn delete(&self, id: &str) -> Result<(), StorageError> {
        self.data
            .borrow_mut()
            .remove(id)
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
        Ok(())
    }

    fn exists(&self, id: &str) -> bool {
        self.data.borrow().contains_key(id)
    }

    fn list_ids(&self) -> Result<Vec<String>, StorageError> {
        Ok(self.data.borrow().keys().cloned().collect())
    }

    // Override default implementation for efficiency - direct access to all values
    fn list(&self) -> Result<Vec<PaletteEntry>, StorageError> {
        Ok(self.data.borrow().values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::{Palette, PresetType};

    #[test]
    fn test_save_and_load() {
        let storage = InMemoryStorage::new();
        let entry = PaletteEntry::new_custom("Test".to_string(), Palette::red());

        storage.save(&entry).unwrap();
        let loaded = storage.load(entry.id.as_str()).unwrap();

        assert_eq!(loaded.id, entry.id);
        assert_eq!(loaded.name, "Test");
    }

    #[test]
    fn test_delete() {
        let storage = InMemoryStorage::new();
        let entry = PaletteEntry::new_custom("Test".to_string(), Palette::red());

        storage.save(&entry).unwrap();
        storage.delete(entry.id.as_str()).unwrap();

        assert!(!storage.exists(entry.id.as_str()));
    }

    #[test]
    fn test_list_multiple_entries() {
        let storage = InMemoryStorage::new();
        let entry1 = PaletteEntry::new_custom("First".to_string(), Palette::red());
        std::thread::sleep(std::time::Duration::from_millis(2));
        let entry2 = PaletteEntry::new_custom("Second".to_string(), Palette::green());

        storage.save(&entry1).unwrap();
        storage.save(&entry2).unwrap();

        let list = storage.list().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_save_overwrites_existing() {
        let storage = InMemoryStorage::new();
        let entry =
            PaletteEntry::new_preset(PresetType::Red, "Original".to_string(), Palette::red());
        let mut updated = entry.clone();
        updated.name = "Updated".to_string();

        storage.save(&entry).unwrap();
        storage.save(&updated).unwrap();

        assert_eq!(storage.list().unwrap().len(), 1);
        assert_eq!(storage.load(entry.id.as_str()).unwrap().name, "Updated");
    }
}
