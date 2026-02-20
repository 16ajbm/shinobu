pub type HexColour = String;

pub fn hex_colour_to_rgb(colour: &HexColour) -> String {
    colour.chars().take(7).collect()
}

pub fn rgb_to_hex_colour(rgb: &str) -> HexColour {
    format!("{}ff", rgb)
}
