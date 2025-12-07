pub struct PaletteRegistry {
    palettes: HashMap<String, Palette>, // Single map since palette contains its type
    persistence: Box<dyn PalettePersistence>,
}

impl PaletteRegistry {
    pub fn new(persistence: Box<dyn PalettePersistence>) -> Self {
        let mut registry = Self {
            palettes: HashMap::new(),
            persistence,
        };

        // Register built-in presets
        let red_palette = Palette::red();
        let green_palette = Palette::green();
        registry.palettes.insert(red_palette.name.clone(), red_palette);
        registry.palettes.insert(green_palette.name.clone(), green_palette);

        // Load previously saved custom palettes
        registry.load_custom_palettes().unwrap_or_else(|e| {
            eprintln!("Failed to load custom palettes: {}", e);
        });

        registry
    }

    pub fn add_palette(&mut self, palette: Palette) -> Result<(), PaletteError> {
        // Validation
        self.validate_palette(&palette)?;
        self.validate_name(&palette.name)?;

        // Only save custom palettes to persistence
        if palette.palette_type == PaletteType::Custom {
            self.persistence.save_palette(&palette.name, &palette)?;
        }

        // Add to runtime registry
        self.palettes.insert(palette.name.clone(), palette);

        Ok(())
    }

    pub fn get_palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(name)
    }

    pub fn list_all_palettes(&self) -> Vec<&Palette> {
        self.palettes.values().collect()
    }

    pub fn list_presets(&self) -> Vec<&Palette> {
        self.palettes
            .values()
            .filter(|p| p.palette_type == PaletteType::Preset)
            .collect()
    }

    pub fn list_custom(&self) -> Vec<&Palette> {
        self.palettes
            .values()
            .filter(|p| p.palette_type == PaletteType::Custom)
            .collect()
    }
}