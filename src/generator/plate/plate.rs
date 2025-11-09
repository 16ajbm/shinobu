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
