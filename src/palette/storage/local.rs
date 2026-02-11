use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

use super::{PaletteStorage, StorageError};
use crate::palette::Palette;

/// LocalStorage-based storage for browser environments.
/// Stores palettes in browser's localStorage with key prefix "shinobu_palette_".
#[derive(Debug, Clone)]
pub struct LocalStorageAdapter {
    prefix: String,
}

impl LocalStorageAdapter {
    pub fn new() -> Self {
        Self {
            prefix: "shinobu_palette_".to_string(),
        }
    }

    /// Get the browser's localStorage object.
    fn get_storage(&self) -> Result<web_sys::Storage, StorageError> {
        window()
            .ok_or(StorageError::PermissionDenied)?
            .local_storage()
            .map_err(|_| StorageError::PermissionDenied)?
            .ok_or(StorageError::PermissionDenied)
    }

    /// Make a full storage key from a palette ID.
    fn make_key(&self, id: &str) -> String {
        format!("{}{}", self.prefix, id)
    }
}

impl Default for LocalStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PaletteStorage for LocalStorageAdapter {
    fn save(&self, palette: &Palette) -> Result<(), StorageError> {
        let storage = self.get_storage()?;
        let key = self.make_key(&palette.name);

        let json = serde_json::to_string(palette)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;

        storage.set_item(&key, &json).map_err(|e| {
            // Check if quota exceeded
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                if err.name() == "QuotaExceededError" {
                    return StorageError::QuotaExceeded;
                }
            }
            StorageError::PermissionDenied
        })?;

        Ok(())
    }

    fn load(&self, name: &str) -> Result<Palette, StorageError> {
        let storage = self.get_storage()?;
        let key = self.make_key(name);

        let json = storage
            .get_item(&key)
            .map_err(|_| StorageError::PermissionDenied)?
            .ok_or_else(|| StorageError::NotFound(name.to_string()))?;

        serde_json::from_str(&json).map_err(|e| StorageError::SerializationError(e.to_string()))
    }

    fn delete(&self, name: &str) -> Result<(), StorageError> {
        let storage = self.get_storage()?;
        let key = self.make_key(name);

        // Check if exists first
        if !self.exists(name) {
            return Err(StorageError::NotFound(name.to_string()));
        }

        storage
            .remove_item(&key)
            .map_err(|_| StorageError::PermissionDenied)?;

        Ok(())
    }

    fn exists(&self, name: &str) -> bool {
        if let Ok(storage) = self.get_storage() {
            let key = self.make_key(name);
            storage.get_item(&key).ok().flatten().is_some()
        } else {
            false
        }
    }

    fn list_names(&self) -> Result<Vec<String>, StorageError> {
        let storage = self.get_storage()?;
        let mut names = Vec::new();

        let length = storage
            .length()
            .map_err(|_| StorageError::PermissionDenied)?;

        for i in 0..length {
            if let Ok(Some(key)) = storage.key(i) {
                if key.starts_with(&self.prefix) {
                    // Strip prefix to get palette name
                    names.push(key[self.prefix.len()..].to_string());
                }
            }
        }

        Ok(names)
    }

    fn list(&self) -> Result<Vec<Palette>, StorageError> {
        let names = self.list_names()?;
        let mut palettes = Vec::new();

        for name in names {
            match self.load(&name) {
                Ok(palette) => palettes.push(palette),
                Err(StorageError::NotFound(_)) => continue, // Skip missing palettes
                Err(e) => return Err(e),
            }
        }

        Ok(palettes)
    }
}

// Note: Tests for WASM code require wasm-pack test or similar
// For now, InMemoryStorage tests provide coverage of the trait contract
