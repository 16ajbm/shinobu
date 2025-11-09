use crate::generator::point::Point;
use crate::generator::HexColour;

// Dots are the individual coloured elements that make up a plate. Although they are called "dots",
// they can come in a variety of shapes.
#[derive(Debug, Clone)]
pub struct Dot {
    pub position: Point,
    pub size: u32,
    pub color: HexColour,
    pub shape: String,
}
