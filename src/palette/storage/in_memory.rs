//! Storage implementation that keeps palettes in memory. This is useful for testing and as a fallback when other storage options are unavailable.

use std::cell::RefCell;
use std::collections::HashMap;

use super::{Palette, PaletteStorage, StorageError};
use crate::palette::{PaletteId, PaletteName};

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
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl PaletteStorage for InMemoryStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        // Returns `Some()` if the key already exists, which we ignore since we want to overwrite it.
        self.data
            .borrow_mut()
            .insert(palette.id.clone(), palette.clone());
        Ok(())
    }

    fn delete(&self, id: &PaletteId) -> Result<(), StorageError> {
        self.data
            .borrow_mut()
            .remove(id)
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;
        Ok(())
    }

    fn exists(&self, id: &PaletteId) -> bool {
        self.data.borrow().contains_key(id)
    }

    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError> {
        Ok(self.data.borrow().keys().cloned().collect())
    }

    fn load(&self, id: &PaletteId) -> Result<Palette, StorageError> {
        self.data
            .borrow()
            .get(id)
            .cloned()
            .ok_or_else(|| StorageError::NotFound(id.to_string()))
    }

    fn load_all(&self) -> Result<Vec<Palette>, StorageError> {
        Ok(self.data.borrow().values().cloned().collect())
    }
}
