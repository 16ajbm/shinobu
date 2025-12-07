pub struct PaletteService {
    registry: Arc<Mutex<PaletteRegistry>>,
}

impl PaletteService {
    pub fn new(persistence: Box<dyn PalettePersistence>) -> Self {
        Self {
            registry: Arc::new(Mutex::new(PaletteRegistry::new(persistence))),
        }
    }

    pub async fn create_custom_palette(&self, name: String, colors: Vec<String>) -> Result<(), ServiceError> {
        let palette = Palette::from_hex_colors(colors)?;

        let mut registry = self.registry.lock().await;
        registry.add_custom_palette(name, palette)?;

        Ok(())
    }

    pub async fn get_all_palettes(&self) -> Result<Vec<PaletteInfo>, ServiceError> {
        let registry = self.registry.lock().await;
        Ok(registry.list_all_palettes())
    }

    pub async fn duplicate_palette(&self, source_name: &str, new_name: String) -> Result<(), ServiceError> {
        let registry = self.registry.lock().await;
        let source_palette = registry.get_palette(source_name)
            .ok_or(ServiceError::PaletteNotFound)?
            .clone();

        drop(registry); // Release lock before calling add_custom_palette

        self.create_custom_palette(new_name, source_palette.to_hex_colors()).await?;
        Ok(())
    }
}