use dioxus::prelude::*;

use crate::components::toast::Toast;
use crate::palette::{Palette, PaletteId};

use super::{PaletteEditor, PalettePreview, PaletteSelector};

static X_ICON: Asset = asset!("/assets/x-icon.svg");

#[derive(Props, PartialEq, Clone, Debug)]
pub struct PaletteManagerProps {
    pub palettes: Vec<Palette>,
    pub selected_primary_id: Option<PaletteId>,
    pub selected_secondary_id: Option<PaletteId>,

    pub on_close: EventHandler<()>,
    pub on_create: EventHandler<Palette>,
    pub on_update: EventHandler<Palette>,
    pub on_delete: EventHandler<PaletteId>,

    pub on_select_primary: EventHandler<PaletteId>,
    pub on_select_secondary: EventHandler<PaletteId>,
    pub toast: Signal<Option<Toast>>,
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

// Remove button
// replace with consistent palette creator up
// List of palettes below with 3 button options for primary, secondary, delete or edit
// How will we display to users that one is selected as primary and one as secondary?

#[component]
pub fn PaletteManager(props: PaletteManagerProps) -> Element {
    let mut mode = use_signal(|| PaletteManagerState::Browse);
    let mut focused_palette_id = use_signal(|| None::<PaletteId>);


    
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

                header { position: "relative",
                    h2 { "Manage Palettes" }
                    div {
                        class: "close-button",
                        position: "absolute",
                        top: "8px",
                        right: "8px",
                        button {

                            "aria-label": "Close palette manager",
                            onclick: move |_| props.on_close.call(()),
                            img {
                                width: "16px",
                                height: "16px",
                                src: "{X_ICON}",
                                alt: "Close",
                            }
                        }
                    }

                    PaletteEditor {
                        toast: props.toast,
                        on_save: move |palette| { props.on_create.call(palette) },
                        on_cancel: use_callback(|_| {}),
                    }

                }

                div { class: "palette-manager-body",
                    section { class: "palette-manager-list",
                        PaletteList {
                            palettes: props.palettes,
                            on_delete: props.on_delete,
                            on_select_edit: props.on_,
                        }
                    }

                // section { class: "palette-manager-detail",
                //     if let Some(palette) = detail_palette {
                //         h3 { "{palette.display_name}" }
                //         PalettePreview { palette: palette.clone() }

                //         p {
                //             if palette.is_preset {
                //                 "Preset palette (read-only)"
                //             } else {
                //                 "Custom palette"
                //             }
                //         }

                //         div { class: "palette-manager-detail-actions",
                //             button {
                //                 onclick: {
                //                     let id = palette.id.clone();
                //                     move |_| props.on_select_primary.call(id.clone())
                //                 },
                //                 "Primary"
                //             }
                //             button {
                //                 onclick: {
                //                     let id = palette.id.clone();
                //                     move |_| props.on_select_secondary.call(id.clone())
                //                 },
                //                 "Secondary"
                //             }
                //         }
                //     } else {
                //         p { "Select a palette to view details." }
                //     }
                // }
                }
            }
        }
    }
}

// ControlGroup { title: "Colour Palettes",
//                 PaletteSelector {
//                     label: "Primary".to_string(),
//                     selected: config().primary,
//                     options: available_palettes,
//                     on_change: move |val| { config.with_mut(|c| c.primary = val) },
//                 }
//                 PaletteSelector {
//                     label: "Secondary (Edges)".to_string(),
//                     selected: config().secondary,
//                     options: available_palettes,
//                     on_change: move |val| { config.with_mut(|c| c.secondary = val) },
//                 }
//             }

#[component]
fn PaletteList(
    palettes: Vec<Palette>,
    on_delete: EventHandler<Palette>,
    on_select_edit: EventHandler<Palette>,
    on_select_primary: EventHandler<Palette>,
    on_select_secondary: EventHandler<Palette>,
) -> Element {
    rsx!(
        for palette in palettes.iter() {
            PaletteEntry {
                palette: palette.clone(),
                on_delete: on_delete.clone(),
                on_select_edit: on_select_edit.clone(),
                on_select_primary: on_select_primary.clone(),
                on_select_secondary: on_select_secondary.clone(),
            }
        }
    )
}

#[component]
fn PaletteEntry(
    palette: Palette,
    on_delete: EventHandler<Palette>,
    on_select_edit: EventHandler<Palette>,
    on_select_primary: EventHandler<Palette>,
    on_select_secondary: EventHandler<Palette>,
) -> Element {
    // TODO - figure out the cloning problem here
    let display_name = &palette.display_name;
    let on_select_edit_palette = palette.clone();
    let on_delete_palette = palette.clone();
    let on_select_primary_palette = palette.clone();
    let on_select_secondary_palette = palette.clone();

    rsx!(
        PalettePreview { palette: palette.clone() }
        button {
            "aria-label": "Select {display_name} as  primary palette",
            onclick: move |_| on_select_primary.call(on_select_primary_palette.clone()),

        }
        button {
            "aria-label": "Select {display_name} as secondary palette",
            onclick: move |_| on_select_secondary.call(on_select_secondary_palette.clone()),
        }
        button {
            "aria-label": "Edit {display_name}",
            onclick: move |_| on_select_edit.call(on_select_edit_palette.clone()),
        }
        button {
            "aria-label": "Delete {display_name}",
            onclick: move |_| on_delete.call(on_delete_palette.clone()),
        }
    )
}
