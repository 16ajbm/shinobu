use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct PaletteId(String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PresetType {
    Red,
    Green,
    Blue,
    Grayscale,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)] // Used in Step 2 storage layer
pub enum PaletteIdType {
    Preset(PresetType),
    Custom(u64),
}

impl PaletteId {
    // Constructors
    pub fn preset(preset_type: PresetType) -> Self {
        PaletteId(format!("preset_{}", preset_type.as_str()))
    }
    pub fn generate_custom() -> Self {
        PaletteId(format!(
            "custom_{}",
            crate::utils::time::current_timestamp_ms()
        ))
    }
    pub fn from_custom_timestamp(timestamp: u64) -> Self {
        PaletteId(format!("custom_{}", timestamp))
    }

    // Parsing
    /// Parse the ID type for pattern matching (Preset or Custom).
    pub fn kind(&self) -> Option<PaletteIdType> {
        if let Some(preset_str) = self.0.strip_prefix("preset_") {
            PresetType::from_str(preset_str).map(PaletteIdType::Preset)
        } else if let Some(ts_str) = self.0.strip_prefix("custom_") {
            ts_str.parse::<u64>().ok().map(PaletteIdType::Custom)
        } else {
            None
        }
    }

    // Queries
    pub fn is_preset(&self) -> bool {
        self.0.starts_with("preset_")
    }
    pub fn is_custom(&self) -> bool {
        self.0.starts_with("custom_")
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PresetType {
    pub fn as_str(&self) -> &str {
        match self {
            PresetType::Red => "red",
            PresetType::Green => "green",
            PresetType::Blue => "blue",
            PresetType::Grayscale => "grayscale",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(PresetType::Red),
            "green" => Some(PresetType::Green),
            "blue" => Some(PresetType::Blue),
            "grayscale" => Some(PresetType::Grayscale),
            _ => None,
        }
    }

    pub fn all() -> [PresetType; 4] {
        [
            PresetType::Red,
            PresetType::Green,
            PresetType::Blue,
            PresetType::Grayscale,
        ]
    }
}

// Display implementations for debugging
impl std::fmt::Display for PaletteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for PresetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_id_format() {
        let id = PaletteId::preset(PresetType::Red);
        assert_eq!(id.as_str(), "preset_red");
        assert!(id.is_preset());
        assert!(!id.is_custom());
    }

    #[test]
    fn test_preset_id_stability() {
        // Preset IDs should be deterministic
        let id1 = PaletteId::preset(PresetType::Green);
        let id2 = PaletteId::preset(PresetType::Green);
        assert_eq!(id1, id2);
        assert_eq!(id1.as_str(), "preset_green");
    }

    #[test]
    fn test_custom_id_format() {
        let id = PaletteId::from_custom_timestamp(1234567890);
        assert_eq!(id.as_str(), "custom_1234567890");
        assert!(!id.is_preset());
        assert!(id.is_custom());
    }

    #[test]
    fn test_custom_id_uniqueness() {
        let id1 = PaletteId::generate_custom();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = PaletteId::generate_custom();
        // IDs should be different (timestamp-based)
        assert_ne!(id1.as_str(), id2.as_str());
    }

    #[test]
    fn test_palette_id_kind_invalid() {
        let id = PaletteId("invalid_format".to_string());
        assert!(id.kind().is_none());
    }

    #[test]
    fn test_preset_type_roundtrip() {
        for preset in PresetType::all() {
            let s = preset.as_str();
            let parsed = PresetType::from_str(s).unwrap();
            assert_eq!(preset, parsed);
        }
    }

    #[test]
    fn test_preset_type_from_str_invalid() {
        assert!(PresetType::from_str("purple").is_none());
        assert!(PresetType::from_str("").is_none());
        assert!(PresetType::from_str("RED").is_none()); // case-sensitive
    }

    #[test]
    fn test_palette_id_serde() {
        let id = PaletteId::preset(PresetType::Red);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"preset_red\"");

        let deserialized: PaletteId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_preset_type_display() {
        assert_eq!(format!("{}", PresetType::Red), "red");
        assert_eq!(format!("{}", PresetType::Grayscale), "grayscale");
    }

    #[test]
    fn test_palette_id_display() {
        let id = PaletteId::preset(PresetType::Green);
        assert_eq!(format!("{}", id), "preset_green");
    }
}
