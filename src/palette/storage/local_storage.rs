//! Storage implementation that saves palettes to a browser's local storage.

use wasm_bindgen::JsCast;
use web_sys::window;

use super::{Palette, PaletteId, PaletteStorage, StorageError};
use crate::palette::PaletteName;

#[derive(Debug, Clone)]
pub struct LocalStorage {
    prefix: String,
}

impl LocalStorage {
    pub fn new(prefix: String) -> Self {
        // The prefix is for distinguishing entitities related to the application
        // across the flat key-value store of localStorage. If anything, it's an
        // exercise.
        Self { prefix }
    }

    pub fn prefix_key(&self, id: &PaletteId) -> String {
        format!("{}{}", self.prefix, id)
    }

    pub fn strip_prefix(&self, key: &String) -> PaletteId {
        key.replace(&self.prefix, "")
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
        let key = self.prefix_key(&palette.id);

        let json = serde_json::to_string_pretty(palette)?;

        storage.set_item(&key, &json).map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        })?;

        Ok(())
    }

    fn delete(&self, id: &PaletteId) -> Result<(), StorageError> {
        let storage = self.get_storage()?;
        let key = self.prefix_key(&id);

        if !self.exists(id) {
            return Err(StorageError::NotFound(id.to_string()));
        }

        storage.remove_item(&key).map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        })?;

        Ok(())
    }

    fn exists(&self, id: &PaletteId) -> bool {
        let key = self.prefix_key(&id);

        if let Ok(storage) = self.get_storage() {
            return storage.get_item(&key).ok().is_some();
        }
        false
    }

    fn list_names(&self) -> Result<Vec<PaletteName>, StorageError> {
        let storage = self.get_storage()?;
        let length = storage.length().map_err(|e| {
            if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                return StorageError::Error(err.name());
            }
            StorageError::PermissionDenied
        })?;

        Ok((0..length)
            .filter_map(|i| storage.key(i).ok().flatten())
            .filter(|key| key.starts_with(&self.prefix))
            .map(|key| self.strip_prefix(&key))
            .collect())
    }

    fn load(&self, id: &PaletteId) -> Result<Palette, StorageError> {
        let storage = self.get_storage()?;
        let key = self.prefix_key(&id);

        let json = storage
            .get_item(&key)
            .map_err(|e| {
                if let Some(err) = e.dyn_ref::<web_sys::DomException>() {
                    return StorageError::Error(err.name());
                }
                StorageError::PermissionDenied
            })?
            .ok_or_else(|| StorageError::NotFound(id.to_string()))?;

        let palette: Palette = serde_json::from_str(&json)?;

        Ok(palette)
    }

    fn load_all(&self) -> Result<Vec<Palette>, StorageError> {
        let names = self.list_names()?;

        Ok(names
            .iter()
            .filter_map(|name| self.load(name).ok())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils;

    use fake::{Fake, Faker};

    fn local_storage() -> LocalStorage {
        LocalStorage::new(Faker.fake())
    }

    #[test]
    fn prefix_key_adds_prefix() {
        let palette = test_utils::palette();
        let storage = local_storage();

        let prefixed_key = storage.prefix_key(&palette.id);

        assert!(prefixed_key.starts_with(&storage.prefix))
    }

    #[test]
    fn strip_prefix_removes_prefix() {
        let palette = test_utils::palette();
        let storage = local_storage();

        let prefixed_key = storage.prefix_key(&palette.id);

        assert!(!storage
            .strip_prefix(&prefixed_key)
            .starts_with(&storage.prefix))
    }
}
