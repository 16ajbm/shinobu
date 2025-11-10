use super::{ControlGroup, SliderControl};
use dioxus::prelude::*;

#[component]
pub fn ControlPanel(
    mut dot_size: Signal<u32>,
    size_jitter: Signal<f32>,
    spacing_multiplier: Signal<f32>,
) -> Element {
    let dot_size_f32 = use_signal(|| dot_size() as f32);

    // Sync f32 slider back to u32 signal (split into two effects to avoid read/write in same scope)
    use_effect(move || {
        let new_value = dot_size_f32() as u32;
        dot_size.set(new_value);
    });

    rsx! {
        aside {
            style: "width: 25%; min-width: 280px; max-width: 600px; padding: 20px; background: white; border-right: 1px solid #ccc; overflow-y: auto; font-size: clamp(16px, 1.5vw, 20px);",

            h2 {
                style: "font-size: clamp(22px, 2.5vw, 28px); margin-bottom: 16px;",
                "Plate Controls"
            }

            ControlGroup {
                title: "Dot Sizing",
                SliderControl {
                    label: "Base Size",
                    value: dot_size_f32,
                    min: 5.0,
                    max: 30.0,
                    step: 1.0,
                    precision: 0,
                }
                SliderControl {
                    label: "Size Variation",
                    value: size_jitter,
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                }
            }

            ControlGroup {
                title: "Spacing",
                SliderControl {
                    label: "Multiplier",
                    value: spacing_multiplier,
                    min: 0.5,
                    max: 2.0,
                }
            }
        }
    }
}
