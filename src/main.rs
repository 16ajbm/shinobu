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

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use crate::palette::Palette;

    let dot_size = use_signal(|| 10u32);
    let size_jitter = use_signal(|| 0.3f32);
    let spacing_multiplier = use_signal(|| 1.0f32);

    // Temporary: use preset palettes until UI is ready
    let primary = Palette::red();
    let secondary = Palette::green();

    let svg_result =
        use_memo(move || create_plate(dot_size(), size_jitter(), spacing_multiplier(), &primary, &secondary));

    rsx! {
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
        }
    }
}
