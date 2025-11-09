pub mod svg_painter;

pub mod poisson;

pub mod colour_source;

pub mod palette;

use crate::{generator::colour_source::EdgeBasedColourSource, image_processor::ImageBuffer};

use self::colour_source::ColourSource;
pub use self::palette::Palette;
use self::svg_painter::SvgPainter;

// Hexadecimal color representation
type HexColour = String;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

// Dots are the individual coloured elements that make up a plate. Although they are called "dots",
// they can come in a variety of shapes.
#[derive(Debug, Clone)]
pub struct Dot {
    pub position: Point,
    pub size: u32,
    pub color: HexColour,
    pub shape: String,
}

#[derive(Debug, Clone)]
pub struct Plate {
    pub dots: Vec<Dot>,
    pub config: PlateConfig,
}

impl Plate {
    pub fn new(config: PlateConfig) -> Self {
        Self {
            dots: Vec::new(),
            config,
        }
    }

    pub fn from_points(
        points: Vec<Point>,
        config: PlateConfig,
        colour_source: &dyn ColourSource,
    ) -> Self {
        let dots = points
            .into_iter()
            .map(|point| Dot {
                color: colour_source.sample_colour(point.x, point.y),
                position: point,
                size: config.dot_size,
                shape: "circle".to_string(),
            })
            .collect();

        Self { dots, config }
    }
}

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

pub struct Generator {
    painter: Box<dyn PlatePainter>,
}

pub trait PlatePainter {
    fn paint(&self, plate: Plate) -> String;
}

impl Generator {
    pub fn new() -> Self {
        Self {
            painter: Box::new(SvgPainter),
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

        Plate::from_points(points, config, &color_source)
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
