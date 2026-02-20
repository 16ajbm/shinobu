use svg::node::element::{Circle, Rectangle};
use svg::Document;

use crate::generator::Dot;
use crate::generator::Plate;

pub struct SvgPainter;

impl super::Painter for SvgPainter {
    fn paint(&self, plate: Plate) -> String {
        let mut document = setup_document(plate.config.width, plate.config.height);

        if let Some(bg_colour) = &plate.config.background {
            let background = Rectangle::new()
                .set("width", plate.config.width)
                .set("height", plate.config.height)
                .set("fill", bg_colour.as_str());
            document = document.add(background);
        }

        for dot in plate.dots.iter() {
            let circle = draw_circle(dot.clone());
            document = document.add(circle);
        }

        document.to_string()
    }
}

fn setup_document(width: u32, height: u32) -> Document {
    Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "none")
        .set("preserveAspectRatio", "xMidYMid meet")
}

fn draw_circle(dot: Dot) -> Circle {
    Circle::new()
        .set("cx", dot.position.x)
        .set("cy", dot.position.y)
        .set("r", dot.size)
        .set("fill", dot.colour)
}
