pub mod colour_source;
pub mod painter;
pub mod point;
pub mod poisson;

use crate::generator::colour_source::EdgeBasedColourSource;
use crate::image_processor::ImageBuffer;
use crate::palette::Palette;

pub use self::point::Point;
use crate::config::PlateConfig;
use crate::hex_colour::HexColour;
use crate::plate::{Dot, Plate};

const DEFAULT_IMAGE_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/extras/seven.png"));

pub fn load_default_edge_map() -> Result<ImageBuffer<u8>, String> {
    ImageBuffer::from_bytes(DEFAULT_IMAGE_BYTES).map_err(|e| format!("Failed to load image: {}", e))
}

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

        let colour_source =
            EdgeBasedColourSource::new(config.width, config.height, edge_map, &primary, &secondary);

        let mut plate = Plate::from_points(points, config, &colour_source);
        plate.fill_gaps(15, &colour_source);
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

pub fn create_plate(
    dot_size: u32,
    size_jitter: f32,
    spacing_multiplier: f32,
    primary: &Palette,
    secondary: &Palette,
) -> Result<String, String> {
    use crate::config::PlateConfig;
    use crate::generator::{poisson::PoissonSampler, Generator};

    let edge_map = load_default_edge_map()?;

    let config = PlateConfig {
        width: edge_map.width,
        height: edge_map.height,
        dot_size,
        size_jitter,
        spacing_multiplier,
        primary: primary.clone(),
        secondary: secondary.clone(),
        background: None,
    };

    let mut sampler = PoissonSampler::new(
        edge_map.width as f32,
        edge_map.height as f32,
        config.poisson_radius(),
        30,
    );
    let points = sampler.generate_samples();

    Ok(Generator::new().generate_and_paint(points, &edge_map, config))
}
