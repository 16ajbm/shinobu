use wasm_bindgen::{JsCast, JsValue};
use web_sys::window ;

use super::{Palette, PaletteStorage, StorageError};

#[derive(Debug, Clone)]
pub struct LocalStorage {
    key: String,
}

impl LocalStorage {
    pub fn new(key: String) -> Self {
        Self { key }
    }


}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new("shinobu_palettes".to_string())
    }
}

impl PaletteStorage for LocalStorage {
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