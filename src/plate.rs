use crate::config::PlateConfig;
use crate::generator::{colour_source::ColourSource, point::Point};
use crate::hex_colour::HexColour;

/// Dots are the individual coloured elements that make up a plate. Although they are called "dots",
/// they will come in a variety of shapes.
#[derive(Debug, Clone)]
pub struct Dot {
    pub position: Point,
    pub size: u32,
    pub colour: HexColour,
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
            .map(|point| {
                let size = config.random_dot_size();
                Dot {
                    colour: colour_source.sample_colour(point.x, point.y, size as f32),
                    position: point,
                    size,
                }
            })
            .collect();

        Self { dots, config }
    }

    pub fn fill_gaps(&mut self, attempts: usize, colour_source: &dyn ColourSource) {
        for _ in 0..attempts {
            let position = self.random_position();

            if let Some(max_radius) = self.max_dot_size_at(&position) {
                let desired_size = self.config.random_dot_size();
                let actual_size = desired_size.min(max_radius);

                if self.can_place_dot_at(&position, actual_size) {
                    let dot = self.create_dot(position, actual_size, colour_source);
                    self.dots.push(dot);
                }
            }
        }
    }

    /// Calculate the maximum dot radius that can fit at a position without overlapping.
    fn max_dot_size_at(&self, position: &Point) -> Option<u32> {
        let available_radius = self.available_radius_at(position)?;
        let max_radius = available_radius / self.config.spacing_multiplier;

        if max_radius >= self.config.min_dot_size() as f32 {
            Some(max_radius as u32)
        } else {
            None
        }
    }

    /// Calculate the available radius at a position based on the closest existing dot.
    fn available_radius_at(&self, position: &Point) -> Option<f32> {
        self.dots
            .iter()
            .map(|dot| {
                let distance_to_center = Self::distance(&dot.position, position);
                distance_to_center - dot.size as f32
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Check if a dot of given size can be placed at position without overlap.
    fn can_place_dot_at(&self, position: &Point, new_dot_radius: u32) -> bool {
        self.dots.iter().all(|existing_dot| {
            let min_distance =
                (new_dot_radius + existing_dot.size) as f32 * self.config.spacing_multiplier;
            let distance = Self::distance(&existing_dot.position, position);
            distance >= min_distance
        })
    }

    /// Calculate Euclidean distance between two points.
    fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }

    fn create_dot(&self, position: Point, size: u32, colour_source: &dyn ColourSource) -> Dot {
        let colour = colour_source.sample_colour(position.x, position.y, size as f32);
        Dot {
            position,
            size,
            colour,
        }
    }

    /// Generate a random position within the plate bounds
    fn random_position(&self) -> Point {
        Point {
            x: rand::random_range(0.0..self.config.width as f32),
            y: rand::random_range(0.0..self.config.height as f32),
        }
    }
}

impl Default for Plate {
    fn default() -> Self {
        Self::new(PlateConfig::default())
    }
}
