use crate::generator::HexColour;
use crate::image_processor::{GrayScaleBuffer, STRONG_EDGE};
use crate::palette::Palette;

pub trait ColourSource {
    fn sample_colour(&self, x: f32, y: f32, radius: f32) -> HexColour;
}

pub struct EdgeBasedColourSource<'a> {
    pub edge_map: &'a GrayScaleBuffer,
    pub primary: &'a Palette,
    pub secondary: &'a Palette,
    pub width: u32,
    pub height: u32,
}

impl<'a> EdgeBasedColourSource<'a> {
    pub fn new(
        width: u32,
        height: u32,
        edge_map: &'a GrayScaleBuffer,
        primary: &'a Palette,
        secondary: &'a Palette,
    ) -> Self {
        Self {
            edge_map,
            primary,
            secondary,
            width,
            height,
        }
    }
}

impl<'a> ColourSource for EdgeBasedColourSource<'a> {
    fn sample_colour(&self, x: f32, y: f32, radius: f32) -> HexColour {
        let sample_points = [
            (x, y),
            (x + radius, y),
            (x - radius, y),
            (x, y + radius),
            (x, y - radius),
        ];

        let overlaps_edge = sample_points.iter().any(|&(sx, sy)| {
            let px = (sx as u32).clamp(0, self.width - 1);
            let py = (sy as u32).clamp(0, self.height - 1);
            self.edge_map.get_pixel(px, py) == STRONG_EDGE
        });

        let palette = if overlaps_edge {
            self.primary
        } else {
            self.secondary
        };
        palette.random_colour()
    }
}
