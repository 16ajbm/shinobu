mod components;
mod config;
mod generator;
mod image_processor;
mod palette;
mod plate;

#[cfg(feature = "fullstack")]
mod server;

use dioxus::prelude::*;

use crate::components::ControlPanel;
use crate::config::PlateConfig;
use crate::generator::create_plate;
use crate::palette::Palette;

static MAIN_CSS: Asset = asset!("/assets/styling.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Initialize palettes
    let available_palettes = use_signal(Palette::load_all);


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

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { style: "display: flex; height: 100vh; margin: 0;",

            ControlPanel { config, available_palettes }

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
        }
    }
}
