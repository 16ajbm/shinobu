mod components;
mod generator;
mod image_processor;
mod palette;
mod utils;

#[cfg(feature = "fullstack")]
mod server;

use dioxus::prelude::*;

use crate::components::{ControlPanel, Toast, ToastContainer, ToastType};
use crate::generator::create_plate;
use crate::palette::{use_persisted_palette, Palette};

#[cfg(not(target_arch = "wasm32"))]
use crate::palette::storage::{FileSystemStorage, PaletteStorage};

#[cfg(target_arch = "wasm32")]
use crate::palette::storage::{LocalStorageAdapter, PaletteStorage};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let dot_size = use_signal(|| 10u32);
    let size_jitter = use_signal(|| 0.3f32);
    let spacing_multiplier = use_signal(|| 1.0f32);

    // Toast notification state
    let mut toast = use_signal(|| None::<Toast>);

    // Initialize storage
    #[cfg(not(target_arch = "wasm32"))]
    let storage = use_signal(|| FileSystemStorage::new().ok());

    #[cfg(target_arch = "wasm32")]
    let storage = use_signal(|| Some(LocalStorageAdapter::new()));

    let mut available_palettes = use_signal(move || {
        let mut palettes = Palette::all_presets();
        if let Some(ref store) = storage() {
            if let Ok(custom) = store.list() {
                palettes.extend(custom);
            }
        }
        palettes
    });

    let presets = Palette::all_presets();

    let primary_palette =
        use_persisted_palette("shinobu_primary_palette", storage, presets[0].clone());

    let secondary_palette =
        use_persisted_palette("shinobu_secondary_palette", storage, presets[1].clone());

    // Handle saving new custom palettes
    let handle_palette_save = move |palette: Palette| {
        if let Some(ref store) = storage() {
            if store.save(&palette).is_ok() {
                let mut palettes = Palette::all_presets();
                if let Ok(custom) = store.list() {
                    palettes.extend(custom);
                }
                available_palettes.set(palettes);
                toast.set(Some(Toast {
                    message: format!("Palette '{}' saved successfully!", palette.name),
                    toast_type: ToastType::Success,
                }));
            } else {
                toast.set(Some(Toast {
                    message: "Failed to save palette".to_string(),
                    toast_type: ToastType::Error,
                }));
            }
        }
    };

    // Handle deleting custom palettes
    let handle_palette_delete = move |name: String| {
        if let Some(ref store) = storage() {
            if store.delete(&name).is_ok() {
                let mut palettes = Palette::all_presets();
                if let Ok(custom) = store.list() {
                    palettes.extend(custom);
                }
                available_palettes.set(palettes);
                toast.set(Some(Toast {
                    message: "Palette deleted successfully".to_string(),
                    toast_type: ToastType::Success,
                }));
            } else {
                toast.set(Some(Toast {
                    message: "Failed to delete palette".to_string(),
                    toast_type: ToastType::Error,
                }));
            }
        }
    };

    // Handle exporting palette as JSON
    let handle_palette_export = move |palette: Palette| {
        if let Ok(json) = serde_json::to_string_pretty(&palette) {
            #[cfg(target_arch = "wasm32")]
            {
                // Browser: trigger download
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        // Create a blob and download link
                        let blob_parts = js_sys::Array::new();
                        blob_parts.push(&wasm_bindgen::JsValue::from_str(&json));

                        if let Ok(blob) = web_sys::Blob::new_with_str_sequence(&blob_parts) {
                            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                                if let Ok(Some(element)) = document.create_element("a").ok() {
                                    let anchor: web_sys::HtmlAnchorElement =
                                        element.dyn_into().unwrap();
                                    anchor.set_href(&url);
                                    anchor.set_download(&format!("{}.json", palette.name));
                                    let _ = anchor.click();
                                    web_sys::Url::revoke_object_url(&url).ok();
                                    toast.set(Some(Toast {
                                        message: format!("Exported '{}' successfully", palette.name),
                                        toast_type: ToastType::Success,
                                    }));
                                }
                            }
                        }
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Native: save to downloads or show dialog
                if let Some(home) = dirs::home_dir() {
                    let path = home.join("Downloads").join(format!("{}.json", palette.name));
                    if std::fs::write(&path, json).is_ok() {
                        toast.set(Some(Toast {
                            message: format!("Exported to Downloads/{}.json", palette.name),
                            toast_type: ToastType::Success,
                        }));
                    } else {
                        toast.set(Some(Toast {
                            message: "Failed to export palette".to_string(),
                            toast_type: ToastType::Error,
                        }));
                    }
                }
            }
        }
    };

    let svg_result = use_memo(move || {
        create_plate(
            dot_size(),
            size_jitter(),
            spacing_multiplier(),
            &primary_palette(),
            &secondary_palette(),
        )
    });

    rsx! {
        div {
            style: "display: flex; height: 100vh; margin: 0; position: relative;",

            ControlPanel {
                dot_size,
                size_jitter,
                spacing_multiplier,
                primary_palette,
                secondary_palette,
                available_palettes,
                on_palette_save: handle_palette_save,
                on_palette_delete: handle_palette_delete,
                on_palette_export: handle_palette_export,
            }

            match svg_result() {
                Ok(svg_content) => rsx! {
                    div {
                        style: "flex: 1; overflow: auto; background: #f0f0f0;",
                        dangerous_inner_html: "{svg_content}"
                    }
                },
                Err(e) => rsx! {
                    div {
                        style: "flex: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; color: #d32f2f;",
                        h2 { "Generation Error" }
                        p { "{e}" }
                    }
                }
            }

            // Toast notification with auto-dismiss
            ToastContainer { toast }
        }
    }
}
