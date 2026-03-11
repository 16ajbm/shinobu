use std::array;

use fake::{Fake, Faker};
use proptest::prelude::*;
use uuid::Uuid;

use crate::hex_colour::HexColour;
use crate::palette::{Palette, PaletteId};

pub fn hex_colour_strategy() -> impl Strategy<Value = HexColour> {
    prop_oneof![
        // 8-char with alpha
        "[0-9a-fA-F]{8}".prop_map(|s| format!("#{}", s)),
        "[0-9a-fA-F]{6}".prop_map(|s| format!("#{}", s)),
    ]
}

pub fn palette() -> Palette {
    Palette {
        id: id(),
        display_name: Faker.fake(),
        colours: array::from_fn(|_| hex_colour()),
        is_preset: false,
    }
}

pub fn hex_colour() -> HexColour {
    format!("#{:08x}", rand::random::<u32>())
}

pub fn id() -> PaletteId {
    Uuid::new_v4().to_string()
}
