use proptest::prelude::*;

use crate::hex_colour::HexColour;

pub fn hex_colour() -> impl Strategy<Value = HexColour> {
    prop_oneof![
        // 8-char with alpha
        "[0-9a-fA-F]{8}".prop_map(|s| format!("#{}", s)),
        "[0-9a-fA-F]{6}".prop_map(|s| format!("#{}", s)),
    ]
}
