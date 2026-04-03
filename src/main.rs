mod components;
mod config;
mod generator;
mod hex_colour;
#[allow(dead_code)]
mod image_processor;
mod palette;
mod plate;
#[cfg(test)]
mod test_utils;

use dioxus::prelude::*;

use crate::components::toast::{Toast, ToastContainer, ToastType};
use crate::components::{
    ControlPanel, ControlPanelProps, PaletteManager, PaletteManagerProps, PlatePreview,
    PlatePreviewProps, Root,
};
use crate::config::PlateConfig;
use crate::generator::create_plate;
use crate::palette::{storage, Palette, PaletteId};

static MAIN_CSS: Asset = asset!("/assets/styling.css");
static PALETTE_ICON: Asset = asset!("/assets/palette-icon.svg");
static PLATE_ICON: Asset = asset!("/assets/plate-icon.svg");
static SETTINGS_ICON: Asset = asset!("/assets/settings-icon.svg");

fn main() {
    dioxus::launch(App);
}

// Root w/ children
// 1. Left sidebar (ControlPanel)
// 2. Main content (SVG plate)
// 3. Palette Manager (modal)

#[component]
fn App() -> Element {
    // Initialize palettes
    let mut available_palettes = use_signal(Palette::load_all);

    let mut control_panel_toggle = use_signal(|| true);

    let mut palette_manager_toggle = use_signal(|| false);

    let mut advanced_settings_toggle = use_signal(|| false);

    // Create config signal and sync with persisted palettes
    let mut config = use_signal(|| {
        let palettes = available_palettes();
        PlateConfig {
            width: 800,
            height: 600,
            dot_size: 10,
            size_jitter: 0.3,
            spacing_multiplier: 1.0,
            primary: palettes[0].clone(),
            secondary: palettes[1].clone(),
            background: None,
        }
    });

    let mut toast: Signal<Option<Toast>> = use_signal(|| None);

    let control_panel_props = ControlPanelProps {
        config,
        on_close: use_callback(move |_| control_panel_toggle.set(false)),
        on_save_palette: use_callback(move |palette: Palette| {
            if let Err(err) = storage::save(&palette) {
                toast.set(Some(Toast::error(format!(
                    "Failed to save palette: {}",
                    err
                ))));
                return;
            }
            available_palettes.set(Palette::load_all());
            toast.set(Some(Toast::success(
                "Palette saved successfully!".to_string(),
            )))
        }),
        _on_delete_palette: use_callback(move |id: PaletteId| {
            if let Err(err) = storage::delete(&id) {
                toast.set(Some(Toast::error(format!(
                    "Failed to delete palette: {}",
                    err
                ))));
                return;
            }
            available_palettes.set(Palette::load_all());
            toast.set(Some(Toast::success(
                "Palette deleted successfully!".to_string(),
            )))
        }),
        available_palettes,
        toast,
    };

    let plate_preview_props = PlatePreviewProps {
        plate_config: config(),
    };

    let palette_manager_props = PaletteManagerProps {
        palettes: available_palettes(),
        selected_primary_id: Some(config().primary.id.clone()),
        selected_secondary_id: Some(config().secondary.id.clone()),
        on_close: use_callback(move |_| palette_manager_toggle.set(false)),
        on_create: use_callback(move |palette: Palette| {
            if let Err(err) = storage::save(&palette) {
                toast.set(Some(Toast::error(format!(
                    "Failed to save palette: {}",
                    err
                ))));

                return;
            }
            available_palettes.set(Palette::load_all());
        }),
        on_update: use_callback(move |palette: Palette| {
            if let Err(err) = storage::save(&palette) {
                toast.set(Some(Toast::error(format!(
                    "Failed to save palette: {}",
                    err
                ))));
                return;
            }
            available_palettes.set(Palette::load_all());
        }),
        on_delete: use_callback(move |id: PaletteId| {
            if let Err(err) = storage::delete(&id) {
                toast.set(Some(Toast::error(format!(
                    "Failed to delete palette: {}",
                    err
                ))));
                return;
            }
            available_palettes.set(Palette::load_all());
        }),
        on_select_primary: use_callback(move |id: PaletteId| {
            if let Some(palette) = available_palettes()
                .into_iter()
                .find(|palette| palette.id == id)
            {
                config.with_mut(|config| config.primary = palette);
            }
        }),
        on_select_secondary: use_callback(move |id: PaletteId| {
            if let Some(palette) = available_palettes()
                .into_iter()
                .find(|palette| palette.id == id)
            {
                config.with_mut(|config| config.secondary = palette);
            }
        }),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Root {
            control_panel_toggle,
            palette_manager_toggle,
            control_panel_props,
            palette_manager_props,
            plate_preview_props,
        }

        ToastContainer { toast }

        // if control_panel_toggle() {
        //     div { style: "display: flex; height: 100vh; margin: 0;",

        //         ControlPanel {
        //             config,
        //             on_save_palette: move |palette: Palette| {
        //                 if let Err(err) = storage::save(&palette) {
        //                     toast
        //                         .set(
        //                             Some(Toast::error(format!("Failed to save palette: {}", err))),
        //                         );
        //                     return;
        //                 }
        //                 available_palettes.set(Palette::load_all());
        //                 toast
        //                     .set(
        //                         Some(Toast::success("Palette saved successfully!".to_string())),
        //                     )
        //             },
        //             _on_delete_palette: move |id: PaletteId| {
        //                 if let Err(err) = storage::delete(&id) {
        //                     toast
        //                         .set(
        //                             Some(Toast::error(format!("Failed to delete palette: {}", err))),
        //                         );
        //                     return;
        //                 }
        //                 available_palettes.set(Palette::load_all());
        //                 toast
        //                     .set(
        //                         Some(Toast::success("Palette deleted successfully!".to_string())),
        //                     )
        //             },
        //             available_palettes,
        //             toast,
        //         }
        //     }
        // } else {
        //     button {
        //         style: "position: absolute; top: 16px; right: 16px; z-index: 20;",
        //         onclick: move |_| control_panel_toggle.set(true),
        //         img { src: "{PLATE_ICON}", width: "24", height: "24" }
        //     }
        // }

        // div { style: "flex: 1; position: relative; overflow: auto; background: #f0f0f0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;",

        //     if !palette_manager_toggle() {
        //         button {
        //             style: "position: absolute; top: 16px; right: 16px; z-index: 20;",
        //             onclick: move |_| palette_manager_toggle.set(true),
        //             img { src: "{PALETTE_ICON}", width: "24", height: "24" }
        //         }
        //     }

        //     match svg_result() {
        //         Ok(svg_content) => rsx! {
        //             div { dangerous_inner_html: "{svg_content}" }
        //         },
        //         Err(e) => rsx! {
        //             div { style: "display: flex; align-items: center; justify-content: center; flex-direction: column; color: #d32f2f;",
        //                 h2 { "Generation Error" }
        //                 p { "{e}" }
        //             }
        //         },
        //     }
        // }

        // if palette_manager_toggle() {
        //     PaletteManager {
        //         palettes: available_palettes(),
        //         selected_primary_id: Some(config().primary.id.clone()),
        //         selected_secondary_id: Some(config().secondary.id.clone()),
        //         on_close: move |_| palette_manager_toggle.set(false),
        //         on_create: move |palette: Palette| {
        //             if let Err(err) = storage::save(&palette) {
        //                 toast
        //                     .set(
        //                         Some(Toast::error(format!("Failed to save palette: {}", err))),
        //                     );

        //                 return;
        //             }
        //             available_palettes.set(Palette::load_all());
        //         },
        //         on_update: move |palette: Palette| {
        //             if let Err(err) = storage::save(&palette) {
        //                 toast
        //                     .set(
        //                         Some(Toast::error(format!("Failed to save palette: {}", err))),
        //                     );
        //                 return;
        //             }
        //             available_palettes.set(Palette::load_all());
        //         },
        //         on_delete: move |id: PaletteId| {
        //             if let Err(err) = storage::delete(&id) {
        //                 toast
        //                     .set(
        //                         Some(Toast::error(format!("Failed to delete palette: {}", err))),
        //                     );
        //                 return;
        //             }
        //             available_palettes.set(Palette::load_all());
        //         },
        //         on_select_primary: move |id: PaletteId| {
        //             if let Some(palette) = available_palettes()
        //                 .into_iter()
        //                 .find(|palette| palette.id == id)
        //             {
        //                 config.with_mut(|config| config.primary = palette);
        //             }
        //         },
        //         on_select_secondary: move |id: PaletteId| {
        //             if let Some(palette) = available_palettes()
        //                 .into_iter()
        //                 .find(|palette| palette.id == id)
        //             {
        //                 config.with_mut(|config| config.secondary = palette);
        //             }
        //         },
        //     }
        // }
    }
}
