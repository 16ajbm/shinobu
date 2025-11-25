use super::{Palette, PaletteEntry, PresetType};
use crate::generator::HexColour;

#[allow(dead_code)] // Used in Step 2 storage layer
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_presets_are_valid() {
        let presets = PaletteManager::presets();
        assert_eq!(presets.len(), 2);

        for preset in presets {
            assert!(preset.is_preset);
            assert!(preset.id.is_preset());
        }
    }

    #[test]
    fn test_create_custom() {
        let manager = PaletteManager::new();
        let colors = [
            "#ff0000ff".to_string(),
            "#ff1111ff".to_string(),
            "#ff2222ff".to_string(),
            "#ff3333ff".to_string(),
            "#ff4444ff".to_string(),
            "#ff5555ff".to_string(),
        ];

        let entry = manager.create_custom("My Custom".to_string(), colors);

        assert_eq!(entry.name, "My Custom");
        assert!(!entry.is_preset);
        assert!(entry.id.is_custom());
    }

    proptest! {
        #[test]
        fn prop_validate_name_accepts_valid_lengths(s in "[a-zA-Z0-9][a-zA-Z0-9 ]{0,49}") {
            // Must start with non-whitespace, 1-50 chars total
            prop_assert!(PaletteManager::validate_name(&s).is_ok());
        }

        #[test]
        fn prop_validate_name_rejects_too_long(s in "[a-zA-Z]{51,100}") {
            prop_assert!(PaletteManager::validate_name(&s).is_err());
        }

        #[test]
        fn prop_validate_name_rejects_whitespace_only(s in "[ \\t\\n]+") {
            prop_assert!(PaletteManager::validate_name(&s).is_err());
        }
    }
}
