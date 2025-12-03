mod components;
mod generator;
mod image_processor;
mod palette;
mod utils;

#[cfg(feature = "fullstack")]
mod server;

use dioxus::prelude::*;

use crate::components::ControlPanel;
use crate::generator::create_plate;
use crate::palette::{PaletteEntry, PaletteManager};

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
    let mut toast_message = use_signal(|| None::<String>);

    // Initialize storage
    #[cfg(not(target_arch = "wasm32"))]
    let storage = use_signal(|| FileSystemStorage::new().ok());

    #[cfg(target_arch = "wasm32")]
    let storage = use_signal(|| Some(LocalStorageAdapter::new()));

    // Load available palettes (presets + saved custom palettes)
    let mut available_palettes = use_signal(move || {
        let mut palettes = PaletteManager::presets();
        if let Some(ref store) = storage() {
            if let Ok(custom) = store.list() {
                palettes.extend(custom);
            }
        }
        palettes
    });

    // Load persisted selected palettes or default to presets
    let primary_palette = use_signal(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native: Load from a config file or use preset
            PaletteManager::presets()[0].clone()
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Web: Try to load from localStorage
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(id)) = storage.get_item("shinobu_primary_palette") {
                        if let Some(ref store) = storage() {
                            if let Ok(entry) = store.load(&id) {
                                return entry;
                            }
                        }
                    }
                }
            }
            PaletteManager::presets()[0].clone()
        }
    });

    let secondary_palette = use_signal(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            PaletteManager::presets()[1].clone()
        }
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(id)) = storage.get_item("shinobu_secondary_palette") {
                        if let Some(ref store) = storage() {
                            if let Ok(entry) = store.load(&id) {
                                return entry;
                            }
                        }
                    }
                }
            }
            PaletteManager::presets()[1].clone()
        }
    });

    // Persist palette selection changes
    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("shinobu_primary_palette", primary_palette().id.as_str());
                }
            }
        }
    });

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item("shinobu_secondary_palette", secondary_palette().id.as_str());
                }
            }
        }
    });

    // Handle saving new custom palettes
    let handle_palette_save = move |entry: PaletteEntry| {
        if let Some(ref store) = storage() {
            if store.save(&entry).is_ok() {
                // Reload available palettes
                let mut palettes = PaletteManager::presets();
                if let Ok(custom) = store.list() {
                    palettes.extend(custom);
                }
                available_palettes.set(palettes);
                toast_message.set(Some(format!("✓ Palette '{}' saved!", entry.name)));
            } else {
                toast_message.set(Some("✗ Failed to save palette".to_string()));
            }
        }
    };

    // Handle deleting custom palettes
    let handle_palette_delete = move |id: String| {
        if let Some(ref store) = storage() {
            if store.delete(&id).is_ok() {
                // Reload available palettes
                let mut palettes = PaletteManager::presets();
                if let Ok(custom) = store.list() {
                    palettes.extend(custom);
                }
                available_palettes.set(palettes);
                toast_message.set(Some("✓ Palette deleted".to_string()));
            } else {
                toast_message.set(Some("✗ Failed to delete palette".to_string()));
            }
        }
    };

    // Handle exporting palette as JSON
    let handle_palette_export = move |entry: PaletteEntry| {
        if let Ok(json) = serde_json::to_string_pretty(&entry) {
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
                                    let anchor: web_sys::HtmlAnchorElement = element.dyn_into().unwrap();
                                    anchor.set_href(&url);
                                    anchor.set_download(&format!("{}.json", entry.name));
                                    let _ = anchor.click();
                                    web_sys::Url::revoke_object_url(&url).ok();
                                    toast_message.set(Some(format!("✓ Exported '{}'", entry.name)));
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
                    let path = home.join("Downloads").join(format!("{}.json", entry.name));
                    if std::fs::write(&path, json).is_ok() {
                        toast_message.set(Some(format!("✓ Exported to {:?}", path)));
                    } else {
                        toast_message.set(Some("✗ Failed to export".to_string()));
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
            &primary_palette().palette,
            &secondary_palette().palette,
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

            // Toast notification (click to dismiss)
            if let Some(message) = toast_message() {
                div {
                    style: "position: fixed; bottom: 20px; right: 20px; background: #323232; color: white; padding: 12px 20px; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.3); font-size: 14px; z-index: 1000; cursor: pointer;",
                    onclick: move |_| toast_message.set(None),
                    "{message}"
                }
            }
        }
    }
}
