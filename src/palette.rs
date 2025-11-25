use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::generator::HexColour;

pub mod entry;
pub mod id;
pub mod manager;

pub use entry::PaletteEntry;
// Used in Step 2 (storage layer)
#[allow(unused_imports)]
pub use id::{PaletteId, PaletteIdType, PresetType};
#[allow(unused_imports)]
pub use manager::PaletteManager;

const COLOURS_PER_PALETTE: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Palette {
    colors: [HexColour; COLOURS_PER_PALETTE],
}

impl Palette {
    pub fn from_colour(colour: HexColour) -> Self {
        Self {
            colors: std::array::from_fn(|_| colour.clone()),
        }
    }

    pub fn from_colours(colours: [HexColour; COLOURS_PER_PALETTE]) -> Self {
        Self { colors: colours }
    }

    pub fn random_colour(&self) -> HexColour {
        let index = rand::rng().random_range(0..COLOURS_PER_PALETTE);
        self.colors[index].clone()
    }

    pub fn red() -> Self {
        Self::from_colours([
            "#d32f2fff".to_string(),
            "#c62828ff".to_string(),
            "#e53935ff".to_string(),
            "#f44336ff".to_string(),
            "#ef5350ff".to_string(),
            "#e57373ff".to_string(),
        ])
    }

    pub fn green() -> Self {
        Self::from_colours([
            "#388e3cff".to_string(),
            "#2e7d32ff".to_string(),
            "#43a047ff".to_string(),
            "#4caf50ff".to_string(),
            "#66bb6aff".to_string(),
            "#81c784ff".to_string(),
        ])
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::from_colour("#000000ff".to_string())
    }
}
