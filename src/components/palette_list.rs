use dioxus::prelude::*;

use crate::palette::PaletteEntry;

use super::PalettePreview;

#[component]
pub fn PaletteList(
    palettes: ReadOnlySignal<Vec<PaletteEntry>>,
    on_delete: EventHandler<String>,
    on_export: EventHandler<PaletteEntry>,
) -> Element {
    let custom_palettes: Vec<_> = palettes()
        .into_iter()
        .filter(|p| !p.is_preset)
        .collect();

    if custom_palettes.is_empty() {
        return rsx! {
            p {
                style: "color: #666; font-size: 13px; font-style: italic;",
                "No custom palettes yet. Create one above!"
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",
            for entry in custom_palettes {
                div {
                    key: "{entry.id.as_str()}",
                    style: "padding: 10px; background: white; border: 1px solid #ddd; border-radius: 6px; display: flex; flex-direction: column; gap: 6px;",

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",
                        p {
                            style: "margin: 0; font-weight: 500; font-size: 14px;",
                            "{entry.name}"
                        }
                        div {
                            style: "display: flex; gap: 6px;",
                            button {
                                style: "padding: 4px 12px; background: #2196f3; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: {
                                    let entry = entry.clone();
                                    move |_| on_export.call(entry.clone())
                                },
                                "Export"
                            }
                            button {
                                style: "padding: 4px 12px; background: #f44336; color: white; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;",
                                onclick: move |_| {
                                    on_delete.call(entry.id.as_str().to_string());
                                },
                                "Delete"
                            }
                        }
                    }

                    PalettePreview { palette: entry.palette.clone() }
                }
            }
        }
    }
}
