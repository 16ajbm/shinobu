use crate::palette::Palette;
use crate::services::{ClipboardService, ExportService};
use std::fs;

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeExportService;

#[cfg(not(target_arch = "wasm32"))]
impl ExportService for NativeExportService {
    fn export_palette(&self, palette: &Palette) -> Result<String, String> {
        let json = serde_json::to_string_pretty(palette)
            .map_err(|e| format!("Serialization failed: {}", e))?;

        if let Some(home) = dirs::home_dir() {
            let path = home
                .join("Downloads")
                .join(format!("{}.json", palette.name));
            fs::write(&path, json).map_err(|e| format!("Failed to write file: {}", e))?;
            Ok(format!("Exported to Downloads/{}.json", palette.name))
        } else {
            Err("Cannot access home directory".to_string())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeClipboardService;

#[cfg(not(target_arch = "wasm32"))]
impl ClipboardService for NativeClipboardService {
    fn copy_text(&self, _text: &str) -> Result<(), String> {
        // Could integrate with a clipboard crate like `clipboard` or `arboard`
        // For now, just indicate it's not supported
        Err("Clipboard not supported on native platform".to_string())
    }
}
