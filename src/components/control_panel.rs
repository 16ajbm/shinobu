use super::{ControlGroup, PaletteEditor, PaletteSelector, SliderControl};
use dioxus::prelude::*;

use crate::components::Toast;
use crate::config::PlateConfig;
use crate::palette::Palette;

#[component]
pub fn ControlPanel(
    mut config: Signal<PlateConfig>,
    available_palettes: Signal<Vec<Palette>>,
    toast: Signal<Option<Toast>>,
) -> Element {
    let mut show_editor = use_signal(|| false);

    rsx! {
        aside { class: "control-panel",

            h2 {
                class: "red",
                style: "font-size: clamp(22px, 2.5vw, 28px); margin-bottom: 16px;",
                "Plate Controls"
            }

            ControlGroup { title: "Colour Palettes",
                PaletteSelector {
                    label: "Primary".to_string(),
                    selected: config().primary,
                    options: available_palettes,
                    on_change: move |val| { config.with_mut(|c| c.primary = val) },
                }
                PaletteSelector {
                    label: "Secondary (Edges)".to_string(),
                    selected: config().secondary,
                    options: available_palettes,
                    on_change: move |val| { config.with_mut(|c| c.secondary = val) },
                }
            }

            ControlGroup { title: "Dot Sizing",
                SliderControl {
                    label: "Base Size",
                    value: config().dot_size as f32,
                    min: 5.0,
                    max: 30.0,
                    step: 1.0,
                    precision: 0,
                    on_change: move |val| { config.with_mut(|c| c.dot_size = val as u32) },
                }
                SliderControl {
                    label: "Size Variation",
                    value: config().size_jitter,
                    min: 0.0,
                    max: 1.0,
                    step: 0.01,
                    on_change: move |val| { config.with_mut(|c| c.size_jitter = val) },
                }
            }

            ControlGroup { title: "Spacing",
                SliderControl {
                    label: "Multiplier",
                    value: config().spacing_multiplier,
                    min: 0.5,
                    max: 2.0,
                    on_change: move |val| { config.with_mut(|c| c.spacing_multiplier = val) },
                }
            }
            ControlGroup { title: "Manage Palettes",
                button { onclick: move |_| show_editor.set(true), "Create Custom Palette" }
            }

            if show_editor() {
                PaletteEditor {
                    toast,
                    on_save: move |palette| {
                        show_editor.set(false);
                    },
                    on_cancel: move |_| show_editor.set(false),
                }
            }

        }
    }
}
