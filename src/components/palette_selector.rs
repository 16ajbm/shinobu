use dioxus::prelude::*;

use crate::palette::Palette;

use super::PalettePreview;

#[component]
pub fn PaletteSelector(
    label: String,
    selected: Signal<Palette>,
    options: ReadOnlySignal<Vec<Palette>>,
) -> Element {
    rsx! {
        div { style: "margin-bottom: 16px;",

            label { style: "display: block; margin-bottom: 6px; font-weight: 600; font-size: 0.95em;",
                "{label}"
            }

            // Current selection with preview
            div { style: "padding: 10px; background: #f5f5f5; border-radius: 6px; margin-bottom: 6px; border: 2px solid #ddd;",
                div { style: "font-size: 14px; margin-bottom: 6px; font-weight: 500;",
                    "{selected().name}"
                }
                PalettePreview { palette: selected().clone() }
            }

            // Dropdown selector
            select {
                style: "width: 100%; padding: 8px; border-radius: 4px; border: 1px solid #ccc; font-size: 14px; cursor: pointer;",
                value: "{selected().name}",
                onchange: move |evt| {
                    let selected_name = evt.value();
                    if let Some(palette) = options().iter().find(|p| p.name == selected_name) {
                        selected.set(palette.clone());
                    }
                },
                for palette in options() {
                    option { value: "{palette.name}", "{palette.name}" }
                }
            }
        }
    }
}
