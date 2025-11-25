use serde::{Deserialize, Serialize};

use super::{Palette, PaletteId, PresetType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaletteEntry {
    pub id: PaletteId,
    pub name: String,
    pub palette: Palette,
    pub is_preset: bool,
}

impl PaletteEntry {
    #[allow(dead_code)] // Used in Step 2 storage layer
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_entry_serde() {
        let palette = Palette::red();
        let entry = PaletteEntry::new_preset(PresetType::Red, "Reds".to_string(), palette);

        // Serialize
        let json = serde_json::to_string(&entry).unwrap();

        // Deserialize
        let deserialized: PaletteEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, entry.name);
        assert_eq!(deserialized.is_preset, entry.is_preset);
        assert_eq!(deserialized.id, entry.id);
    }

    #[test]
    fn test_custom_palette_uniqueness() {
        let palette1 = Palette::red();
        let palette2 = Palette::green();

        let entry1 = PaletteEntry::new_custom("Entry 1".to_string(), palette1);
        std::thread::sleep(std::time::Duration::from_millis(2));
        let entry2 = PaletteEntry::new_custom("Entry 2".to_string(), palette2);

        // Custom entries should have different IDs
        assert_ne!(entry1.id, entry2.id);
    }
}
