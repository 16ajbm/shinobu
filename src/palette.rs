pub mod storage;

use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::{hex_colour, hex_colour::HexColour};

const COLOURS_PER_PALETTE: usize = 6;

/// Name of the `Palette` given by the user, not guaranteed to be unique
pub type PaletteName = String;
/// Generated ID of the `Palette`, guaranteed to be unique
pub type PaletteId = String;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum PaletteError {
    #[error(
        "Invalid colour count: expected {}, got different amount",
        COLOURS_PER_PALETTE
    )]
    InvalidColourCount,
    #[error("Invalid colour format: {0}")]
    InvalidColourFormat(String),
    #[error("Name already exists: {0}")]
    NameAlreadyExists(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    pub id: PaletteId,
    pub display_name: PaletteName,
    pub colours: [HexColour; COLOURS_PER_PALETTE],
    pub is_preset: bool,
}

impl Palette {
    /// Returns all available palettes: presets (+ custom palettes in the future)
    pub fn load_all() -> Vec<Palette> {
        let mut palettes = Self::all_presets();

        if let Ok(custom) = storage::load_all() {
            palettes.extend(custom);
        }

        palettes
    }

    /// Create a preset palette from colours array with a stable ID
    pub fn preset(
        id: PaletteId,
        display_name: PaletteName,
        colours: [HexColour; COLOURS_PER_PALETTE],
    ) -> Self {
        for colour in &colours {
            assert!(
                hex_colour::is_valid(colour),
                "Invalid hex colour: '{}'",
                colour
            )
        }

        Self {
            id,
            display_name,
            colours,
            is_preset: true,
        }
    }

    pub fn custom(display_name: PaletteName, colours: [HexColour; COLOURS_PER_PALETTE]) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id,
            display_name,
            colours,
            is_preset: false,
        }
    }

    /// Validate palette name according to business rules
    #[allow(dead_code)]
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            Err("Palette name cannot be empty".to_string())
        } else if name.len() > 50 {
            Err("Palette name too long (max 50 characters)".to_string())
        } else {
            Ok(())
        }
    }

    /// Get all built-in preset palettes
    pub fn all_presets() -> Vec<Self> {
        vec![Self::red(), Self::green(), Self::greyscale()]
    }

    /// Check if this palette can be edited (only custom palettes)
    #[allow(dead_code)]
    pub fn is_editable(&self) -> bool {
        !self.is_preset
    }

    pub fn colours(&self) -> &[HexColour; COLOURS_PER_PALETTE] {
        &self.colours
    }

    pub fn random_colour(&self) -> HexColour {
        let index = rand::random_range(0..COLOURS_PER_PALETTE);
        self.colours[index].clone()
    }

    pub fn red() -> Self {
        Self::preset(
            "preset_red".to_string(),
            "Red".to_string(),
            [
                "#d32f2fff".to_string(),
                "#c62828ff".to_string(),
                "#e53935ff".to_string(),
                "#f44336ff".to_string(),
                "#ef5350ff".to_string(),
                "#e57373ff".to_string(),
            ],
        )
    }

    pub fn green() -> Self {
        Self::preset(
            "preset_green".to_string(),
            "Green".to_string(),
            [
                "#388e3cff".to_string(),
                "#2e7d32ff".to_string(),
                "#43a047ff".to_string(),
                "#4caf50ff".to_string(),
                "#66bb6aff".to_string(),
                "#81c784ff".to_string(),
            ],
        )
    }

    pub fn greyscale() -> Self {
        Self::preset(
            "preset_greyscale".to_string(),
            "Greyscale".to_string(),
            [
                "#212121ff".to_string(),
                "#424242ff".to_string(),
                "#616161ff".to_string(),
                "#9e9e9eff".to_string(),
                "#bdbdbdff".to_string(),
                "#e0e0e0ff".to_string(),
            ],
        )
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::greyscale()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic(expected = "Invalid hex colour")]
    fn test_preset_panics_on_invalid_hex_colour() {
        Palette::preset(
            "id".to_string(),
            "Test".to_string(),
            ["bad"; 6].map(String::from),
        );
    }
}
