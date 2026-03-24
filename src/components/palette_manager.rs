use dioxus::prelude::*;

use crate::palette::{Palette, PaletteId};

use super::PalettePreview;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct PaletteManagerProps {
    pub palettes: Vec<Palette>,
    pub is_open: bool,
    pub selected_primary_id: Option<PaletteId>,
    pub selected_secondary_id: Option<PaletteId>,

    pub on_close: EventHandler<()>,
    pub on_create: EventHandler<Palette>,
    pub on_update: EventHandler<Palette>,
    pub on_delete: EventHandler<PaletteId>,

    pub on_select_primary: EventHandler<PaletteId>,
    pub on_select_secondary: EventHandler<PaletteId>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PaletteManagerState {
    Browse,
    Create,
    Edit(PaletteId),
    Delete(PaletteId),
}

// Reuse
// `palette_preview`
// `palette_editor` (with some modifications to support both creating and editing)
// ListPane
// ListItem
// DeleteConfirmInline

#[component]
pub fn PaletteManager(props: PaletteManagerProps) -> Element {
    let mut mode = use_signal(|| PaletteManagerState::Browse);
    let mut focused_palette_id = use_signal(|| None::<PaletteId>);

    if !props.is_open {
        return rsx!(
            {}
        );
    }

    let detail_palette_id = focused_palette_id()
        .or_else(|| props.selected_primary_id)
        .or_else(|| props.selected_secondary_id);

    let detail_palette = detail_palette_id
        .as_ref()
        .and_then(|id| props.palettes.iter().find(|p| &p.id == id))
        .cloned();

    rsx! {
        div {
            class: "palette-manager-overlay",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "palette-manager-dialog",
                role: "dialog",
                "aria-modal": "true",
                "aria-label": "Manage palettes",
                onclick: move |event| event.stop_propagation(),

                header {
                    h2 { "Manage Palettes" }
                    button {
                        "aria-label": "Close palette manager",
                        onclick: move |_| props.on_close.call(()),
                        "Close"
                    }
                    button {
                        "aria-label": "Create new palette",
                        onclick: move |_| mode.set(PaletteManagerState::Create),
                        "New Palette"
                    }
                }

                div { class: "palette-manager-body",
                    section { class: "palette-manager-list",
                        h3 { "Palettes" }
                        ul {
                            for palette in props.palettes.iter() {
                                li { key: "{palette.id}",
                                    button {
                                        class: if detail_palette_id.as_ref() == Some(&palette.id) { "selected" } else { "" },
                                        onclick: {
                                            let palette_id = palette.id.clone();
                                            move |_| focused_palette_id.set(Some(palette_id.clone()))
                                        },
                                        "{palette.display_name}"
                                    }
                                }
                            }
                        }
                    }

                    section { class: "palette-manager-detail",
                        if let Some(palette) = detail_palette {
                            h3 { "{palette.display_name}" }
                            PalettePreview { palette: palette.clone() }

                            p {
                                if palette.is_preset {
                                    "Preset palette (read-only)"
                                } else {
                                    "Custom palette"
                                }
                            }

                            div { class: "palette-manager-detail-actions",
                                button {
                                    onclick: {
                                        let id = palette.id.clone();
                                        move |_| props.on_select_primary.call(id.clone())
                                    },
                                    "Set as Primary"
                                }
                                button {
                                    onclick: {
                                        let id = palette.id.clone();
                                        move |_| props.on_select_secondary.call(id.clone())
                                    },
                                    "Set as Secondary"
                                }
                            }
                        } else {
                            p { "Select a palette to view details." }
                        }
                    }
                }
            }
        }
    }
}
