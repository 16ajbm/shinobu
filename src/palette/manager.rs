use crate::generator::HexColour;
use crate::palette::{Palette, PaletteEntry, PresetType};

pub struct PaletteManager;

impl PaletteManager {
    pub fn new() -> Self {
        PaletteManager
    }

    pub fn presets() -> Vec<PaletteEntry> {
        vec![
            PaletteEntry::new_preset(PresetType::Red, "Reds".to_string(), Palette::red()),
            PaletteEntry::new_preset(PresetType::Green, "Greens".to_string(), Palette::green()),
        ]
    }

    pub fn create_custom(&self, name: String, colors: [HexColour; 6]) -> PaletteEntry {
        let palette = Palette::from_colours(colors);
        PaletteEntry::new_custom(name, palette)
    }

    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            Err("Palette name cannot be empty".to_string())
        } else if name.len() > 50 {
            Err("Palette name too long (max 50 characters)".to_string())
        } else {
            Ok(())
        }
    }
}
