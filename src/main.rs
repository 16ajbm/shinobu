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
use crate::components::ControlPanel;
use crate::config::PlateConfig;
use crate::generator::create_plate;
use crate::palette::{storage, Palette, PaletteId};

static MAIN_CSS: Asset = asset!("/assets/styling.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize palettes
    let mut available_palettes = use_signal(Palette::load_all);

    // Create config signal and sync with persisted palettes
    let config = use_signal(|| {
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

    let mut toast: Signal<Option<Toast>> = use_signal(|| None);

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { style: "display: flex; height: 100vh; margin: 0;",

            ControlPanel {
                config,
                on_save_palette: move |palette: Palette| {
                    if let Err(err) = storage::save(&palette) {
                        toast
                            .set(
                                Some(Toast {
                                    message: format!("Failed to save palette: {}", err),
                                    toast_type: ToastType::Error,
                                }),
                            );
                        return;
                    }
                    available_palettes.set(Palette::load_all());
                    toast
                        .set(
                            Some(Toast {
                                message: "Palette saved successfully!".to_string(),
                                toast_type: ToastType::Success,
                            }),
                        )
                },
                _on_delete_palette: move |id: PaletteId| {
                    if let Err(err) = storage::delete(&id) {
                        toast
                            .set(
                                Some(Toast {
                                    message: format!("Failed to delete palette: {}", err),
                                    toast_type: ToastType::Error,
                                }),
                            );
                        return;
                    }
                    available_palettes.set(Palette::load_all());
                    toast
                        .set(
                            Some(Toast {
                                message: "Palette deleted successfully!".to_string(),
                                toast_type: ToastType::Success,
                            }),
                        )
                },
                available_palettes,
                toast,
            }

            match svg_result() {
                Ok(svg_content) => rsx! {
                    div {
                        style: "flex: 1; overflow: auto; background: #f0f0f0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;",
                        dangerous_inner_html: "{svg_content}",
                    }
                },
                Err(e) => rsx! {
                    div { style: "flex: 1; display: flex; align-items: center; justify-content: center; flex-direction: column; color: #d32f2f;",
                        h2 { "Generation Error" }
                        p { "{e}" }
                    }
                },
            }
            ToastContainer { toast }
        }
    }
}
