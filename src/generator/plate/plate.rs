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

        for _ in 0..attempts {
            let position = self.random_position(&mut rng);

            // Find the maximum diameter that could fit at this position
            if let Some(max_diameter) = self.max_dot_size_at(&position) {
                // Generate a size respecting both random sizing AND physical constraints
                let desired_size = self.config.random_dot_size();
                let actual_size = desired_size.min(max_diameter);

                // Double-check it actually fits (should always pass now)
                if self.can_place_dot_at(&position, actual_size) {
                    let dot = self.create_dot(position, actual_size, colour_source);
                    self.dots.push(dot);
                }
            }
        }
    }

    /// Calculate the maximum dot diameter that can fit at a position without overlapping.
    fn max_dot_size_at(&self, position: &Point) -> Option<u32> {
        let spacing = self.config.spacing_multiplier;

        // Find the closest existing dot
        let min_distance_to_existing = self
            .dots
            .iter()
            .map(|dot| {
                let dist = Self::distance(&dot.position, position);
                let existing_radius = dot.size as f32 / 2.0;
                // Available radius = distance minus existing radius
                dist - existing_radius
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap())?;

        // Convert to diameter, accounting for spacing
        let max_radius = min_distance_to_existing / spacing;

        if max_radius > 0.5 {
            // At least 1px diameter
            Some((max_radius * 2.0) as u32)
        } else {
            None
        }
    }

    /// Check if a dot of given size can be placed at position without overlap.
    fn can_place_dot_at(&self, position: &Point, new_dot_size: u32) -> bool {
        let new_radius = new_dot_size as f32 / 2.0;
        let spacing = self.config.spacing_multiplier;

        self.dots.iter().all(|existing_dot| {
            let existing_radius = existing_dot.size as f32 / 2.0;
            let min_distance = (new_radius + existing_radius) * spacing;

            let distance = Self::distance(&existing_dot.position, position);
            distance >= min_distance
        })
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
