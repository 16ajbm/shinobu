mod components;
mod config;
mod generator;
mod image_processor;
mod palette;
mod plate;
mod services;
mod utils;

#[cfg(feature = "fullstack")]
mod server;

use dioxus::prelude::*;

use crate::components::{ControlPanel, Toast, ToastContainer, ToastType};
use crate::config::PlateConfig;
use crate::generator::create_plate;
use crate::palette::{use_persisted_palette, Palette};
use crate::services::create_export_service;

#[cfg(not(target_arch = "wasm32"))]
use crate::palette::storage::{FileSystemStorage, PaletteStorage};

#[cfg(target_arch = "wasm32")]
use crate::palette::storage::{LocalStorageAdapter, PaletteStorage};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize with default config and preset palettes
    let presets = Palette::all_presets();

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

    // Load persisted palettes
    let primary_palette =
        use_persisted_palette("shinobu_primary_palette", storage, presets[0].clone());

    let secondary_palette =
        use_persisted_palette("shinobu_secondary_palette", storage, presets[1].clone());

    // Create config signal and sync with persisted palettes
    let mut config = use_signal(|| PlateConfig {
        width: 800,
        height: 600,
        dot_size: 10,
        size_jitter: 0.3,
        spacing_multiplier: 1.0,
        primary: presets[0].clone(),
        secondary: presets[1].clone(),
        background: None,
    });

    // Update config when persisted palettes change
    use_effect(move || {
        config.with_mut(|c| {
            c.primary = primary_palette();
            c.secondary = secondary_palette();
        });
    });

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
        let export_service = create_export_service();
        match export_service.export_palette(&palette) {
            Ok(message) => {
                toast.set(Some(Toast {
                    message,
                    toast_type: ToastType::Success,
                }));
            }
            Err(error) => {
                toast.set(Some(Toast {
                    message: format!("Export failed: {}", error),
                    toast_type: ToastType::Error,
                }));
            }
        }
    };

    let svg_result = use_memo(move || {
        let c = config();
        create_plate(
            c.dot_size,
            c.size_jitter,
            c.spacing_multiplier,
            &c.primary,
            &c.secondary,
        )
    });

    rsx! {
        div {
            style: "display: flex; height: 100vh; margin: 0; position: relative;",

            ControlPanel {
                config,
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
