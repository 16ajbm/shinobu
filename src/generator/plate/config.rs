use crate::generator::HexColour;
use crate::palette::Palette;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct PlateConfig {
    pub dot_size: u32,
    pub width: u32,
    pub height: u32,
    pub size_jitter: f32,
    pub spacing_multiplier: f32,
    pub primary: Palette,
    pub secondary: Palette,
    pub background: Option<HexColour>,
}

impl PlateConfig {
    pub fn new(
        width: u32,
        height: u32,
        dot_size: u32,
        size_jitter: f32,
        spacing_multiplier: f32,
    ) -> Self {
        Self {
            dot_size,
            width,
            height,
            primary: Palette::from_colour("White".to_string(), "#ffffff".to_string()),
            secondary: Palette::default(),
            background: None,
            size_jitter,
            spacing_multiplier,
        }
    }

    /// Calculate Poisson disc sampling radius to prevent dot overlap.
    ///
    /// Returns the minimum distance between dot centers, accounting for
    /// maximum possible dot size and desired spacing.
    pub fn poisson_radius(&self) -> f32 {
        let max_radius = self.dot_size as f32 * (1.0 + self.size_jitter);
        let min_distance = 2.0 * max_radius;
        min_distance * self.spacing_multiplier
    }

    /// Generate a random dot size within jitter bounds.
    pub fn random_dot_size(&self) -> u32 {
        let base = self.dot_size as f32;
        let min = (base * (1.0 - self.size_jitter)).max(1.0);
        let max = base * (1.0 + self.size_jitter);
        rand::rng().random_range(min as u32..=max as u32)
    }

    pub fn min_dot_size(&self) -> u32 {
        let base = self.dot_size as f32;
        (base * (1.0 - self.size_jitter)).max(1.0) as u32
    }
}
