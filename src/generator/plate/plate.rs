use rand::Rng;

use crate::generator::{
    colour_source::ColourSource,
    plate::{config::PlateConfig, dot::Dot},
    point::Point,
};

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
                    color: colour_source.sample_colour(point.x, point.y, size as f32),
                    position: point,
                    size,
                    shape: "circle".to_string(),
                }
            })
            .collect();

        Self { dots, config }
    }

    pub fn fill_gaps(&mut self, attempts: usize, colour_source: &dyn ColourSource) {
        let mut rng = rand::rng();
        let width = self.config.width;
        let height = self.config.height;

        for _ in 0..attempts {
            let x = rng.random_range(0.0..width as f32);
            let y = rng.random_range(0.0..height as f32);

            let new_size = self.config.random_dot_size() / 2;
            let new_radius = new_size as f32 / 2.0;

            if self.dots.iter().all(|dot| {
                let existing_radius = dot.size as f32 / 2.0;
                let distance = ((dot.position.x - x).powi(2) + (dot.position.y - y).powi(2)).sqrt();
                // Ensure no overlap with a small safety margin
                distance >= new_radius + existing_radius
            }) {
                let color = colour_source.sample_colour(x, y, new_size as f32);
                self.dots.push(Dot {
                    position: Point { x, y },
                    size: new_size,
                    color,
                    shape: "circle".to_string(),
                });
            }
        }
    }

    /// Calculate Euclidean distance between two points.
    fn distance(p1: &Point, p2: &Point) -> f32 {
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
    }

    fn create_dot(&self, position: Point, size: u32, colour_source: &dyn ColourSource) -> Dot {
        let color = colour_source.sample_colour(position.x, position.y, size as f32);
        Dot {
            position,
            size,
            color,
            shape: "circle".to_string(),
        }
    }

    /// Generate a random position within the plate bounds
    fn random_position(&self, rng: &mut impl Rng) -> Point {
        Point {
            x: rng.random_range(0.0..self.config.width as f32),
            y: rng.random_range(0.0..self.config.height as f32),
        }
    }
}
