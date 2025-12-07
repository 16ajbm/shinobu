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

    pub fn load_all_palettes(storage: &impl super::storage::PaletteStorage) -> Vec<PaletteEntry> {
        let mut palettes = Self::presets();
        if let Ok(custom) = storage.list() {
            palettes.extend(custom);
        }
        palettes
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
    use crate::palette::storage::{InMemoryStorage, PaletteStorage};
    use proptest::prelude::*;

    #[test]
    fn test_presets_count_and_properties() {
        let presets = PaletteManager::presets();
        assert_eq!(presets.len(), 2);
        assert!(presets.iter().all(|p| p.is_preset && p.id.is_preset()));
    }

    #[test]
    fn test_load_all_palettes_with_empty_storage() {
        let storage = InMemoryStorage::new();
        let palettes = PaletteManager::load_all_palettes(&storage);

        assert_eq!(palettes.len(), 2);
        assert!(palettes.iter().all(|p| p.is_preset));
    }

    #[test]
    fn test_load_all_palettes_with_custom() {
        let storage = InMemoryStorage::new();
        let custom = PaletteEntry::new_custom("Custom".to_string(), Palette::red());
        storage.save(&custom).unwrap();

        let palettes = PaletteManager::load_all_palettes(&storage);

        assert_eq!(palettes.len(), 3);
        assert_eq!(palettes.iter().filter(|p| !p.is_preset).count(), 1);
    }

    #[test]
    fn test_validate_name_edge_cases() {
        assert!(PaletteManager::validate_name("a").is_ok());
        assert!(PaletteManager::validate_name(&"x".repeat(50)).is_ok());
        assert!(PaletteManager::validate_name(&"x".repeat(51)).is_err());
        assert!(PaletteManager::validate_name("").is_err());
        assert!(PaletteManager::validate_name("  ").is_err());
        assert!(PaletteManager::validate_name("\t\n").is_err());
    }

    proptest! {
        #[test]
        fn prop_validate_name_length_boundary(s in "[a-zA-Z0-9 ]{1,100}") {
            let result = PaletteManager::validate_name(&s);
            let trimmed_len = s.trim().len();

            if trimmed_len == 0 {
                prop_assert!(result.is_err());
            } else if s.len() <= 50 {
                prop_assert!(result.is_ok());
            } else {
                prop_assert!(result.is_err());
            }
        }
    }

    // Integration tests for the full save/delete workflow
    mod integration {
        use super::*;

        #[test]
        fn test_workflow_save_and_reload() {
            let storage = InMemoryStorage::new();
            let custom = PaletteEntry::new_custom("MyPalette".to_string(), Palette::red());

            // Initial state: only presets
            let palettes_before = PaletteManager::load_all_palettes(&storage);
            assert_eq!(palettes_before.len(), 2);
            assert!(palettes_before.iter().all(|p| p.is_preset));

            // Save custom palette
            storage.save(&custom).unwrap();

            // Reload and verify it appears
            let palettes_after = PaletteManager::load_all_palettes(&storage);
            assert_eq!(palettes_after.len(), 3);
            assert_eq!(palettes_after.iter().filter(|p| !p.is_preset).count(), 1);
            assert!(palettes_after.iter().any(|p| p.name == "MyPalette"));
        }

        #[test]
        fn test_workflow_delete_and_reload() {
            let storage = InMemoryStorage::new();
            let custom1 = PaletteEntry::new_custom("Palette1".to_string(), Palette::red());
            let custom2 = PaletteEntry::new_custom("Palette2".to_string(), Palette::green());

            // Save two custom palettes
            storage.save(&custom1).unwrap();
            storage.save(&custom2).unwrap();

            let palettes_before = PaletteManager::load_all_palettes(&storage);
            assert_eq!(palettes_before.len(), 4); // 2 presets + 2 custom

            // Delete one
            storage.delete(custom1.id.as_str()).unwrap();

            // Reload and verify only one custom remains
            let palettes_after = PaletteManager::load_all_palettes(&storage);
            assert_eq!(palettes_after.len(), 3);
            assert!(!palettes_after.iter().any(|p| p.name == "Palette1"));
            assert!(palettes_after.iter().any(|p| p.name == "Palette2"));
        }

        #[test]
        fn test_workflow_overwrite_palette() {
            let storage = InMemoryStorage::new();
            let original = PaletteEntry::new_custom("Test".to_string(), Palette::red());
            let mut updated = original.clone();
            updated.name = "Updated".to_string();

            // Save original
            storage.save(&original).unwrap();
            let palettes_v1 = PaletteManager::load_all_palettes(&storage);
            assert!(palettes_v1.iter().any(|p| p.name == "Test"));

            // Overwrite with updated version (same ID)
            storage.save(&updated).unwrap();
            let palettes_v2 = PaletteManager::load_all_palettes(&storage);

            // Should still have 3 total (not 4), with updated name
            assert_eq!(palettes_v2.len(), 3);
            assert!(!palettes_v2.iter().any(|p| p.name == "Test"));
            assert!(palettes_v2.iter().any(|p| p.name == "Updated"));
        }

        #[test]
        fn test_workflow_delete_nonexistent() {
            let storage = InMemoryStorage::new();

            // Deleting non-existent palette should error
            let result = storage.delete("nonexistent_id");
            assert!(result.is_err());

            // But shouldn't affect the palette list
            let palettes = PaletteManager::load_all_palettes(&storage);
            assert_eq!(palettes.len(), 2); // Still just presets
        }

        #[test]
        fn test_workflow_presets_not_affected_by_storage() {
            let storage = InMemoryStorage::new();

            // Save a custom with same name as preset (but different ID)
            let fake_reds = PaletteEntry::new_custom("Reds".to_string(), Palette::green());
            storage.save(&fake_reds).unwrap();

            let palettes = PaletteManager::load_all_palettes(&storage);

            // Should have both: the preset "Reds" and custom "Reds"
            assert_eq!(palettes.len(), 3);
            let reds: Vec<_> = palettes.iter().filter(|p| p.name == "Reds").collect();
            assert_eq!(reds.len(), 2);
            assert!(reds.iter().any(|p| p.is_preset));
            assert!(reds.iter().any(|p| !p.is_preset));
        }
    }
}
