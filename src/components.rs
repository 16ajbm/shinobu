mod colour_picker;
mod control_group;
mod control_panel;
mod palette_editor;
mod palette_manager;
mod palette_preview;
mod palette_selector;
mod plate_preview;
mod slider_control;
pub mod toast;

pub use control_group::ControlGroup;
pub use control_panel::{ControlPanel, ControlPanelProps};
pub use palette_editor::PaletteEditor;
pub use palette_manager::{PaletteManager, PaletteManagerProps};
pub use palette_preview::PalettePreview;
pub use palette_selector::PaletteSelector;
pub use plate_preview::{PlatePreview, PlatePreviewProps};
pub use slider_control::SliderControl;

use dioxus::prelude::*;

static PALETTE_ICON: Asset = asset!("/assets/palette-icon.svg");
static PLATE_ICON: Asset = asset!("/assets/plate-icon.svg");

#[component]
pub fn Root(
    mut control_panel_toggle: Signal<bool>,
    mut palette_manager_toggle: Signal<bool>,
    control_panel_props: ControlPanelProps,
    palette_manager_props: PaletteManagerProps,
    plate_preview_props: PlatePreviewProps,
) -> Element {
    rsx! {
        div { class: "root", display: "flex", flex_direction: "row",
            if control_panel_toggle() {

                div {
                    class: "plate-controls-container",
                    width: "300px",
                    padding: "20px",
                    ControlPanel { ..control_panel_props }
                }
            } else {
                ControlPanelButton { control_panel_toggle }
            }

            div { class: "plate-preview-container", flex: "1",
                PlatePreview { ..plate_preview_props }
            }

            if palette_manager_toggle() {
                div { class: "palette-manager-container",
                    PaletteManager { ..palette_manager_props }
                }
            } else {
                PaletteManagerButton { palette_manager_toggle }
            }
        }
    }
}

#[component]
fn PaletteManagerButton(palette_manager_toggle: Signal<bool>) -> Element {
    rsx! {
        div { class: "palette-manager-button",
            button {
                style: "position: absolute; top: 16px; right: 16px; z-index: 20;",
                onclick: move |_| palette_manager_toggle.set(true),
                img { src: "{PALETTE_ICON}", width: "24", height: "24" }
            }
        }
    }
}

#[component]
fn ControlPanelButton(control_panel_toggle: Signal<bool>) -> Element {
    rsx! {
        div { class: "control-panel-button",
            button {
                style: "position: absolute; top: 16px; left: 16px; z-index: 20;",
                onclick: move |_| control_panel_toggle.set(true),
                img { src: "{PLATE_ICON}", width: "24", height: "24" }
            }
        }
    }
}

// Left side plate controls
// Right side palette manager
// center plate preview
