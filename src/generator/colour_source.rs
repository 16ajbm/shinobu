use crate::generator::palette::Palette;
use crate::generator::HexColour;
use crate::image_processor::{GrayScaleBuffer, STRONG_EDGE};

pub trait ColourSource {
    fn sample_colour(&self, x: f32, y: f32) -> HexColour;
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
    fn sample_colour(&self, x: f32, y: f32) -> HexColour {
        let px = (x as u32).min(self.width - 1);
        let py = (y as u32).min(self.height - 1);

        let is_edge = self.edge_map.get_pixel(px, py) == STRONG_EDGE;

        let palette = if is_edge {
            self.primary
        } else {
            self.secondary
        };

        palette.random_colour()
    }
}
