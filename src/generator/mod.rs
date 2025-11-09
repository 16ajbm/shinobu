pub mod colour_source;
pub mod painter;
pub mod palette;
pub mod plate;
pub mod point;
pub mod poisson;

use crate::generator::colour_source::EdgeBasedColourSource;
use crate::image_processor::ImageBuffer;

pub use self::palette::Palette;
pub use self::plate::{Dot, Plate, PlateConfig};
pub use self::point::Point;

pub type HexColour = String;

pub struct Generator {
    painter: Box<dyn painter::Painter>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            painter: Box::new(painter::svg::SvgPainter),
        }
    }

    pub fn paint(&self, plate: Plate) -> String {
        self.painter.paint(plate)
    }

    pub fn generate_from_edges(
        &self,
        points: Vec<Point>,
        edge_map: &ImageBuffer<u8>,
        config: PlateConfig,
    ) -> Plate {
        let primary = config.primary.clone();
        let secondary = config.secondary.clone();

        let color_source =
            EdgeBasedColourSource::new(config.width, config.height, edge_map, &primary, &secondary);

        let mut plate = Plate::from_points(points, config, &color_source);
        plate.fill_gaps(15, &color_source);
        plate
    }

    pub fn generate_and_paint(
        &self,
        points: Vec<Point>,
        edge_map: &ImageBuffer<u8>,
        config: PlateConfig,
    ) -> String {
        let plate = self.generate_from_edges(points, edge_map, config);
        self.paint(plate)
    }
}
