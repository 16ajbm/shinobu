mod hooks;
pub mod storage;

pub use hooks::use_persisted_palette;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::config::HexColour;

const COLOURS_PER_PALETTE: usize = 6;

type PaletteName = String;

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
    pub id: String,
    pub display_name: String,
    pub colours: [HexColour; COLOURS_PER_PALETTE],
    pub is_preset: bool,
}

impl Palette {
    /// Returns all available palettes: presets (+ custom palettes in the future)
    pub fn load_all() -> Vec<Palette> {
        // Extend to custom loading when in place
        Self::all_presets()
    }

    /// Create a preset palette from colours array with a stable ID
    pub fn preset(
        id: String,
        display_name: String,
        colours: [HexColour; COLOURS_PER_PALETTE],
    ) -> Self {
        for colour in &colours {
            assert!(
                Self::is_valid_hex_colour(colour),
                "Invalid hex colour: '{}'",
                colour
            )
        }

        Self {
            id,
            display_name,
            colours: colours,
            is_preset: true,
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

    fn is_valid_hex_colour(colour: &HexColour) -> bool {
        if !colour.starts_with('#') {
            return false;
        }

        let hex = &colour[1..];

        if hex.len() != 6 && hex.len() != 8 {
            return false;
        }

        hex.chars().all(|c| c.is_ascii_hexdigit())
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
        let index = rand::rng().random_range(0..COLOURS_PER_PALETTE);
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
mod test_utils;

#[cfg(test)]
mod tests {
    use crate::palette::test_utils;

    use super::*;
    use proptest::prelude::*;

    // Helper
    // fn assert_valid_hex_colour(colour: &str) {
    // }

    #[test]
    #[should_panic(expected = "Invalid hex colour")]
    fn test_preset_panics_on_invalid_hex_colour() {
        Palette::preset(
            "id".to_string(),
            "Test".to_string(),
            ["bad"; 6].map(String::from),
        );
    }

    proptest!(
        #[test]
        fn test_is_valid_hex_colour(colour in test_utils::hex_colour()) {
            assert!(Palette::is_valid_hex_colour(&colour));
        }
    );
}
