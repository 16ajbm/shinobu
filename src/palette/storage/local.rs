//! Storage implementation that saves palettes to a browser's local storage.

use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

use super::{Palette, PaletteStorage, StorageError};
use crate::palette::PaletteName;

#[derive(Debug, Clone)]
pub struct LocalStorage {
    prefix: String,
}

impl LocalStorage {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn prefix_key(&self, name: &PaletteName) -> String {
        format!("{}{}", self.prefix, name)
    }

    pub fn get_storage(&self) -> Result<web_sys::Storage, StorageError> {
        window()
            .ok_or(StorageError::PermissionDenied)?
            .local_storage()
            .map_err(|_| StorageError::PermissionDenied)?
            .ok_or(StorageError::PermissionDenied)
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self::new("shinobu_palettes".to_string())
    }
}

impl PaletteStorage for LocalStorage {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        let storage = self.get_storage()?;
        let key = self.prefix_key(&palette.name);

        let json = serde_json::to_string_pretty(palette)?;

        storage.set_item(&key, &json).map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                // TODO - something more applicable
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        });

        Ok(())
    }

    fn delete(&self, name: &PaletteName) -> Result<(), StorageError> {
        let storage = self.get_storage()?;
        let key = self.prefix_key(&name);

        if !self.exists(name) {
            return Err(StorageError::NotFound(name.to_string()));
        }

        storage.remove_item(&key).map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                // TODO - something more applicable
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        });

        Ok(())
    }

    fn exists(&self, name: &PaletteName) -> bool {
        let key = self.prefix_key(&name);

        if let Ok(storage) = self.get_storage() {
            return storage.get_item(&key).ok().is_some();
        }
        false
    }

    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        todo!()
    }

    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError> {
        let storage = self.get_storage()?;
        let length = storage.length().map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                // TODO - something more applicable
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        })?;

        Ok(
            (0..length)
            .filter_map(|i| storage.key(i).ok().flatten())
            .filter(|key| key.starts_with(&self.prefix))
            .collect()
        )
    }

    fn load(&self, name: &PaletteName) -> Result<Palette, StorageError> {
        let storage = self.get_storage()?;
        let key = self.prefix_key(&name);

        let json = storage
            .get_item(&key)
            .map_err(|e| {
                if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                    return StorageError::Error(err.name());
                }
                StorageError::PermissionDenied
            })?
            .ok_or_else(|| StorageError::NotFound(key.to_string()))?;

        // TODO - determine how this line can be skipped without having to bring in (unnecessary) deserialization
        let palette: Palette = serde_json::from_str(&json)?;

        Ok(palette)
    }
}
