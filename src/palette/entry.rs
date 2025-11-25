use serde::{Deserialize, Serialize};

use crate::palette::{Palette, PaletteId, PresetType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteEntry {
    pub id: PaletteId,
    pub name: String,
    pub palette: Palette,
    pub is_preset: bool,
}

impl PaletteEntry {
    pub fn new_preset(preset_type: PresetType, name: String, palette: Palette) -> Self {
        Self {
            id: PaletteId::preset(preset_type),
            name,
            palette,
            is_preset: true,
        }
    }

    pub fn new_custom(name: String, palette: Palette) -> Self {
        Self {
            id: PaletteId::generate_custom(),
            name,
            palette,
            is_preset: false,
        }
    }
}
