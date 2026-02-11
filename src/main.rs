mod components;
mod generator;
mod image_processor;

#[cfg(feature = "fullstack")]
mod server;

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

    let svg_result =
        use_memo(move || create_plate(dot_size(), size_jitter(), spacing_multiplier()));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div {
            style: "display: flex; height: 100vh; margin: 0;",

            ControlPanel {
                dot_size,
                size_jitter,
                spacing_multiplier,
            }

            match svg_result() {
                Ok(svg_content) => rsx! {
                    div {
                        style: "flex: 1; overflow: auto; background: #f0f0f0; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center;",
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
        }
    }   
}
