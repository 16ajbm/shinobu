use super::{ControlGroup, PaletteSelector, SliderControl};
use dioxus::prelude::*;

use crate::config::PlateConfig;
use crate::palette::Palette;

#[component]
pub fn ControlPanel(
    mut config: Signal<PlateConfig>,
    primary_palette: Signal<Palette>,
    secondary_palette: Signal<Palette>,
    available_palettes: ReadOnlySignal<Vec<Palette>>,
) -> Element {
    let dot_size_f32 = use_signal(|| config().dot_size as f32);
    let size_jitter = use_signal(|| config().size_jitter);
    let spacing_multiplier = use_signal(|| config().spacing_multiplier);

    // Sync config with palette changes
    use_effect(move || {
        config.with_mut(|c| {
            c.primary = primary_palette();
            c.secondary = secondary_palette();
        });
    });

    // Sync changes back to config and palette signals
    use_effect(move || {
        config.with_mut(|c| {
            c.dot_size = dot_size_f32() as u32;
            c.size_jitter = size_jitter();
            c.spacing_multiplier = spacing_multiplier();
        });
    });

    rsx! {
        aside { class: "control-panel",

            h2 {
                class: "red",
                style: "font-size: clamp(22px, 2.5vw, 28px); margin-bottom: 16px;",
                "Plate Controls"
            }

            ControlGroup { title: "Color Palettes",
                PaletteSelector {
                    label: "Primary".to_string(),
                    selected: primary_palette,
                    options: available_palettes,
                }
                PaletteSelector {
                    label: "Secondary (Edges)".to_string(),
                    selected: secondary_palette,
                    options: available_palettes,
                }
            }

            ControlGroup { title: "Dot Sizing",
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

            ControlGroup { title: "Spacing",
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
