mod components;
mod generator;
mod image_processor;

#[cfg(feature = "fullstack")]
mod server;

use dioxus::html::u::exportparts;
use dioxus::prelude::*;

use crate::components::ControlPanel;
use crate::generator::create_plate;

static MAIN_CSS: Asset = asset!("/assets/styling.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let dot_size = use_signal(|| 10u32);
    let size_jitter = use_signal(|| 0.3f32);
    let spacing_multiplier = use_signal(|| 1.0f32);

    let export_format = use_signal(|| "SVG".to_string());
    let export_size = use_signal(|| "Original".to_string());
    let export_color_space = use_signal(|| "sRGB".to_string());


    let svg_result =
        use_memo(move || create_plate(dot_size(), size_jitter(), spacing_multiplier()));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div {
            class: "app-container",

            ControlPanel {
                dot_size,
                size_jitter,
                spacing_multiplier,
                export_format,
                export_size,
                export_color_space,
            }

            match svg_result() {
                Ok(svg_content) => rsx! {
                    div {
                        class: "svg-container",
                        dangerous_inner_html: "{svg_content}"
                    }
                },
                Err(e) => rsx! {
                    div {
                        class:"error-message",
                        h2 { "Generation Error" }
                        p { "{e}" }
                    }
                }
            }
        }
    }
}
