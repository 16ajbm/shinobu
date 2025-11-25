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
