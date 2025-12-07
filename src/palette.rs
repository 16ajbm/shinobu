use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::config::HexColour;

pub mod hooks;
pub mod storage;

pub use hooks::use_persisted_palette;

const COLOURS_PER_PALETTE: usize = 6;

#[derive(Debug, thiserror::Error)]
pub enum PaletteError {
    #[error(
        "Invalid color count: expected {}, got different amount",
        COLOURS_PER_PALETTE
    )]
    InvalidColorCount,
    #[error("Invalid color format: {0}")]
    InvalidColorFormat(String),
    #[error("Name already exists: {0}")]
    NameAlreadyExists(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    pub name: String,
    pub colors: [HexColour; COLOURS_PER_PALETTE],
    pub is_preset: bool,
}

impl Palette {
    /// Create a custom palette from a single color (fills all slots)
    pub fn from_colour(name: String, colour: HexColour) -> Self {
        Self {
            name,
            colors: std::array::from_fn(|_| colour.clone()),
            is_preset: false,
        }
    }

    /// Create a custom palette from colors array
    pub fn custom(name: String, colours: [HexColour; COLOURS_PER_PALETTE]) -> Self {
        Self {
            name,
            colors: colours,
            is_preset: false,
        }
    }

    /// Create a preset palette from colors array
    pub fn preset(name: String, colours: [HexColour; COLOURS_PER_PALETTE]) -> Self {
        Self {
            name,
            colors: colours,
            is_preset: true,
        }
    }

    /// Validate palette name according to business rules
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
        vec![Self::red(), Self::green()]
    }

    /// Check if this palette can be edited (only custom palettes)
    pub fn is_editable(&self) -> bool {
        !self.is_preset
    }

    pub fn colors(&self) -> &[HexColour; COLOURS_PER_PALETTE] {
        &self.colors
    }

    pub fn random_colour(&self) -> HexColour {
        let index = rand::rng().random_range(0..COLOURS_PER_PALETTE);
        self.colors[index].clone()
    }

    pub fn red() -> Self {
        Self::preset(
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
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            colors: std::array::from_fn(|_| "#000000ff".to_string()),
            is_preset: false,
        }
    }
}
