use std::cell::RefCell;
use std::collections::HashMap;

use super::{Palette, PaletteStorage, StorageError};

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
