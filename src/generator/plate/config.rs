use crate::generator::HexColour;
use crate::generator::Palette;

#[derive(Debug, Clone)]
pub struct PlateConfig {
    pub dot_size: u32,
    pub width: u32,
    pub height: u32,
    pub density: u32,
    pub size_jitter: f32,
    pub position_jitter: f32,
    pub primary: Palette,
    pub secondary: Palette,
    pub background: Option<HexColour>,
}

impl PlateConfig {
    pub fn new(
        width: u32,
        height: u32,
        density: u32,
        dot_size: u32,
        size_jitter: f32,
        position_jitter: f32,
    ) -> Self {
        Self {
            dot_size,
            width,
            height,
            primary: Palette::from_colour("#ffffff".to_string()),
            secondary: Palette::default(),
            background: None,
            density,
            size_jitter,
            position_jitter,
        }
    }
}
