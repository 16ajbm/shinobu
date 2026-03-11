pub type HexColour = String;

pub fn hex_colour_to_rgb(colour: &HexColour) -> String {
    colour.chars().take(7).collect()
}

pub fn default_array() -> [HexColour; 6] {
    ["#000000ff"; 6].map(String::from)
}

pub fn is_valid(colour: &HexColour) -> bool {
    if !colour.starts_with('#') {
        return false;
    }

    let hex = &colour[1..];

    if hex.len() != 6 && hex.len() != 8 {
        return false;
    }

    hex.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn normalize(colour: &str) -> HexColour {
    let mut normalized = colour.trim().to_lowercase();

    if !normalized.starts_with('#') {
        normalized = format!("#{}", normalized);
    }

    normalized
}

#[cfg(test)]
mod tests {
    use crate::test_utils;

    use super::*;
    use proptest::prelude::*;

    proptest!(
        #[test]
        fn test_is_valid(colour in test_utils::hex_colour_strategy()) {
            assert!(is_valid(&colour));
        }

        #[test]
        fn test_normalize(colour in test_utils::hex_colour_strategy()) {
            let unnormalized = &colour[1..];

            assert!(!is_valid(&unnormalized.to_string()));
            assert!(is_valid(&colour));
        }
    );
}
