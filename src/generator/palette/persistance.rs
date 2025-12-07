pub trait PalettePersistence: Send + Sync {
    fn save_palette(&self, name: &str, palette: &Palette) -> Result<(), PersistenceError>;
    fn load_palette(&self, name: &str) -> Result<Palette, PersistenceError>;
    fn list_palettes(&self) -> Result<Vec<String>, PersistenceError>;
    fn delete_palette(&self, name: &str) -> Result<(), PersistenceError>;
}

// Web implementation using browser localStorage
pub struct WebLocalStoragePersistence;

impl PalettePersistence for WebLocalStoragePersistence {
    fn save_palette(&self, name: &str, palette: &Palette) -> Result<(), PersistenceError> {
        let json = serde_json::to_string(palette)?;
        let key = format!("palette_{}", name);

        // Use web_sys to interact with localStorage
        let window = web_sys::window().ok_or(PersistenceError::BrowserNotAvailable)?;
        let storage = window.local_storage()?.ok_or(PersistenceError::StorageNotAvailable)?;
        storage.set_item(&key, &json)?;

        Ok(())
    }

    // ... other methods
}

// Desktop/file-based implementation
pub struct FilePersistence {
    base_path: PathBuf,
}

impl PalettePersistence for FilePersistence {
    fn save_palette(&self, name: &str, palette: &Palette) -> Result<(), PersistenceError> {
        let file_path = self.base_path.join(format!("{}.json", name));
        let json = serde_json::to_string_pretty(palette)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }

    // ... other methods
}