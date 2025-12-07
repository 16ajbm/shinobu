#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use web_sys;

use crate::palette::Palette;
use crate::services::{ClipboardError, ClipboardService, ExportError, ExportService, ServiceError};

#[cfg(target_arch = "wasm32")]
pub struct WebExportService;

#[cfg(target_arch = "wasm32")]
impl ExportService for WebExportService {
    fn export_palette(&self, palette: &Palette) -> Result<String, ExportError> {
        let json =
            serde_json::to_string_pretty(palette).map_err(|e| ExportError::ExportFailed {
                palette_name: palette.name.clone(),
                source: ServiceError::Serialization(e.to_string()),
            })?;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));

                if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&blob_parts) {
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                        if let Ok(element) = document.create_element("a") {
                            let anchor: web_sys::HtmlAnchorElement =
                                element.dyn_into().map_err(|_| ExportError::ExportFailed {
                                    palette_name: palette.name.clone(),
                                    source: ServiceError::Platform(
                                        "Failed to create anchor element".to_string(),
                                    ),
                                })?;
                            anchor.set_href(&url);
                            anchor.set_download(&format!("{}.json", palette.name));
                            let _ = anchor.click();
                            web_sys::Url::revoke_object_url(&url).ok();
                            return Ok(format!("Exported '{}' successfully", palette.name));
                        }
                    }
                }
            }
        }
        Err(ExportError::ExportFailed {
            palette_name: palette.name.clone(),
            source: ServiceError::Platform("Failed to trigger download".to_string()),
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub struct WebClipboardService;

#[cfg(target_arch = "wasm32")]
impl ClipboardService for WebClipboardService {
    fn copy_text(&self, text: &str) -> Result<(), ClipboardError> {
        if let Some(window) = web_sys::window() {
            let nav = window.navigator();
            let clipboard = nav.clipboard();
            let _ = clipboard.write_text(text);
            Ok(())
        } else {
            Err(ClipboardError::CopyFailed {
                source: ServiceError::Platform("No window available".to_string()),
            })
        }
    }
}
