use super::{ControlGroup, PaletteEditor, PaletteList, PaletteSelector, SliderControl};
use dioxus::prelude::*;

use crate::palette::PaletteEntry;

#[component]
pub fn ControlPanel(
    mut dot_size: Signal<u32>,
    size_jitter: Signal<f32>,
    spacing_multiplier: Signal<f32>,
    primary_palette: Signal<PaletteEntry>,
    secondary_palette: Signal<PaletteEntry>,
    available_palettes: ReadOnlySignal<Vec<PaletteEntry>>,
    on_palette_save: EventHandler<PaletteEntry>,
    on_palette_delete: EventHandler<String>,
    on_palette_export: EventHandler<PaletteEntry>,
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
                title: "Color Palettes",
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

            ControlGroup {
                title: "Custom Palettes",
                PaletteEditor {
                    on_save: move |entry| on_palette_save.call(entry)
                }
                div {
                    style: "margin-top: 16px;",
                    p {
                        style: "font-size: 13px; font-weight: 600; margin-bottom: 8px;",
                        "Saved Custom Palettes"
                    }
                    PaletteList {
                        palettes: available_palettes,
                        on_delete: move |id| on_palette_delete.call(id),
                        on_export: move |entry| on_palette_export.call(entry),
                    }
                }
            }
        }
    }
}
