use crate::palette::Palette;
use crate::services::{ClipboardError, ClipboardService, ExportError, ExportService, ServiceError};
use std::fs;

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeExportService;

#[cfg(not(target_arch = "wasm32"))]
impl ExportService for NativeExportService {
    fn export_palette(&self, palette: &Palette) -> Result<String, ExportError> {
        let json =
            serde_json::to_string_pretty(palette).map_err(|e| ExportError::ExportFailed {
                palette_name: palette.name.clone(),
                source: ServiceError::Serialization(e.to_string()),
            })?;

        if let Some(home) = dirs::home_dir() {
            let path = home
                .join("Downloads")
                .join(format!("{}.json", palette.name));
            fs::write(&path, json).map_err(|e| ExportError::ExportFailed {
                palette_name: palette.name.clone(),
                source: ServiceError::Io(e.to_string()),
            })?;
            Ok(format!("Exported to Downloads/{}.json", palette.name))
        } else {
            Err(ExportError::ExportFailed {
                palette_name: palette.name.clone(),
                source: ServiceError::Platform("Cannot access home directory".to_string()),
            })
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeClipboardService;

#[cfg(not(target_arch = "wasm32"))]
impl ClipboardService for NativeClipboardService {
    fn copy_text(&self, _text: &str) -> Result<(), ClipboardError> {
        // Could integrate with a clipboard crate like `clipboard` or `arboard`
        // For now, just indicate it's not supported
        Err(ClipboardError::CopyFailed {
            source: ServiceError::Unsupported(
                "Clipboard not supported on native platform".to_string(),
            ),
        })
    }
}
