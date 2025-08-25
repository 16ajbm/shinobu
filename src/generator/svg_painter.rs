// Generator implementation using the `svg` crate

use crate::generator::{Dot, Plate, PlatePainter};

use svg::node::element::Circle;
use svg::Document;

pub struct SvgPainter;

// String is SVG path
// TODO - find a better way to return reference to artifact
impl PlatePainter for SvgPainter {
    fn paint(&self, plate: Plate) -> String {
        let mut document = Document::new()
            .set("width", plate.config.width)
            .set("height", plate.config.height)
            .set("fill", "none");

        for dot in plate.dots.iter() {
            let circle = draw_dot(dot.clone());
            // Add the circle to the document
            document = document.add(circle);
        }

        // Implementation details go here

        let path = "output.svg";

        svg::save(path, &document).unwrap();

        path.to_string()
    }
}

fn draw_dot(dot: Dot) -> Circle {
    let circle = Circle::new()
        .set("cx", dot.x)
        .set("cy", dot.y)
        .set("r", dot.size)
        .set("fill", dot.color);

    circle
}
