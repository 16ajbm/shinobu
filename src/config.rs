use crate::palette::Palette;

use crate::hex_colour::HexColour;

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
            primary: Palette::red(),
            secondary: Palette::green(),
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
        let (_, max_radius) = self.dot_radius_bounds();
        required_distance(max_radius, max_radius, self.spacing_multiplier)
    }

    pub fn dot_radius_bounds(&self) -> (u32, u32) {
        let base = self.dot_size as f32;
        let jitter = self.size_jitter.clamp(0.0, 1.0);
        let min = (base * (1.0 - jitter)).max(1.0).floor() as u32;
        let max = (base * (1.0 + jitter)).max(min as f32).ceil() as u32;
        (min, max)
    }

    pub fn spacing_multiplier(&self) -> f32 {
        self.spacing_multiplier.max(1.0)
    }

    pub fn required_distance(&self, first_radius: u32, second_radius: u32) -> f32 {
        required_distance(first_radius, second_radius, self.spacing_multiplier)
    }

    /// Generate a random dot size within jitter bounds.
    pub fn random_dot_size(&self) -> u32 {
        let (min, max) = self.dot_radius_bounds();
        rand::random_range(min..=max)
    }

    pub fn min_dot_size(&self) -> u32 {
        self.dot_radius_bounds().0
    }
}

pub fn required_distance(first_radius: u32, second_radius: u32, spacing_multiplier: f32) -> f32 {
    (first_radius + second_radius) as f32 * spacing_multiplier.max(1.0)
}

impl Default for PlateConfig {
    fn default() -> Self {
        Self::new(800, 600, 10, 0.3, 1.0)
    }
}
