use rand::Rng;

use crate::generator::HexColour;

#[derive(Debug, thiserror::Error)]
pub enum PaletteError {
    #[error("Invalid color count: expected {}, got different amount", COLOURS_PER_PALETTE)]
    InvalidColorCount,
    #[error("Invalid color format: {0}")]
    InvalidColorFormat(String),
    #[error("Name already exists: {0}")]
    NameAlreadyExists(String),
}

const COLOURS_PER_PALETTE: usize = 6;

#[derive(Debug, Clone, PartialEq)]
pub enum PaletteType {
    Preset,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub name: String,
    pub colors: [HexColour; COLOURS_PER_PALETTE],
    pub palette_type: PaletteType,
}

impl Palette {
    pub fn from_colour(name: String, colour: HexColour, palette_type: PaletteType) -> Self {
        Self {
            name,
            colors: std::array::from_fn(|_| colour.clone()),
            palette_type,
        }
    }

    pub fn from_colours(name: String, colours: [HexColour; COLOURS_PER_PALETTE], palette_type: PaletteType) -> Self {
        Self {
            name,
            colors: colours,
            palette_type,
        }
    }

    pub fn random_colour(&self) -> HexColour {
        let index = rand::rng().random_range(0..COLOURS_PER_PALETTE);
        self.colors[index].clone()
    }

    pub fn is_editable(&self) -> bool {
        matches!(self.palette_type, PaletteType::Custom)
    }

    pub fn to_hex_colors(&self) -> Vec<String> {
        self.colors.to_vec()
    }

    pub fn from_hex_colors(name: String, colors: Vec<String>) -> Result<Self, PaletteError> {
        if colors.len() != COLOURS_PER_PALETTE {
            return Err(PaletteError::InvalidColorCount);
        }

        let colors_array: [String; COLOURS_PER_PALETTE] = colors
            .try_into()
            .map_err(|_| PaletteError::InvalidColorCount)?;

        Ok(Self::from_colours(name, colors_array, PaletteType::Custom))
    }

    pub fn red() -> Self {
        Self::from_colours(
            "Red".to_string(),
            [
                "#d32f2fff".to_string(),
                "#c62828ff".to_string(),
                "#e53935ff".to_string(),
                "#f44336ff".to_string(),
                "#ef5350ff".to_string(),
                "#e57373ff".to_string(),
            ],
            PaletteType::Preset,
        )
    }

    pub fn green() -> Self {
        Self::from_colours(
            "Green".to_string(),
            [
                "#388e3cff".to_string(),
                "#2e7d32ff".to_string(),
                "#43a047ff".to_string(),
                "#4caf50ff".to_string(),
                "#66bb6aff".to_string(),
                "#81c784ff".to_string(),
            ],
            PaletteType::Preset,
        )
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::from_colour(
            "Default".to_string(),
            "#000000ff".to_string(),
            PaletteType::Custom,
        )
    }
}
