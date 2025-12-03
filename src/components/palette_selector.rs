use dioxus::prelude::*;

use crate::palette::PaletteEntry;

use super::PalettePreview;

#[component]
pub fn PaletteSelector(
    label: String,
    selected: Signal<PaletteEntry>,
    options: ReadOnlySignal<Vec<PaletteEntry>>,
) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 16px;",

            label {
                style: "display: block; margin-bottom: 6px; font-weight: 600; font-size: 0.95em;",
                "{label}"
            }

            // Current selection with preview
            div {
                style: "padding: 10px; background: #f5f5f5; border-radius: 6px; margin-bottom: 6px; border: 2px solid #ddd;",
                div {
                    style: "font-size: 14px; margin-bottom: 6px; font-weight: 500;",
                    "{selected().name}"
                }
                PalettePreview { palette: selected().palette.clone() }
            }

            // Dropdown selector
            select {
                style: "width: 100%; padding: 8px; border-radius: 4px; border: 1px solid #ccc; font-size: 14px; cursor: pointer;",
                value: "{selected().id.as_str()}",
                onchange: move |evt| {
                    let selected_id = evt.value();
                    if let Some(entry) = options().iter().find(|e| e.id.as_str() == selected_id) {
                        selected.set(entry.clone());
                    }
                },
                for entry in options() {
                    option {
                        value: "{entry.id.as_str()}",
                        "{entry.name}"
                    }
                }
            }
        }
    }
}
