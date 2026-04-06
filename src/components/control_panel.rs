use super::{ControlGroup, PaletteEditor, PaletteSelector, SliderControl};
use dioxus::prelude::*;

use crate::components::toast::Toast;
use crate::config::PlateConfig;
use crate::palette::{Palette, PaletteId};

static X_ICON: Asset = asset!("/assets/x-icon.svg");

#[component]
pub fn ControlPanel(
    mut config: Signal<PlateConfig>,
    on_close: EventHandler<()>,
    on_save_palette: EventHandler<Palette>,
    _on_delete_palette: EventHandler<PaletteId>,
    available_palettes: Signal<Vec<Palette>>,
    toast: Signal<Option<Toast>>,
) -> Element {
    rsx! {
        aside { class: "control-panel",

            h2 {
                class: "red",
                style: "font-size: clamp(22px, 2.5vw, 28px); margin-bottom: 16px;",
                "Plate Controls"
                div {
                    class: "close-button",
                    position: "absolute",
                    top: "8px",
                    left: "8px",
                    button {

                        "aria-label": "Close control panel",
                        onclick: move |_| on_close.call(()),
                        img {
                            width: "16px",
                            height: "16px",
                            src: "{X_ICON}",
                            alt: "Close",
                        }
                    }
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
        }
    }
}
