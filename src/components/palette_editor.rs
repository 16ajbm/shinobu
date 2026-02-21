use dioxus::prelude::*;

use crate::components::toast::{Toast, ToastType};
use crate::hex_colour::HexColour;
use crate::palette::Palette;

use super::colour_picker::ColourPicker;
use super::PalettePreview;

#[component]
pub fn PaletteEditor(
    toast: Signal<Option<Toast>>,
    on_save: EventHandler<Palette>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut name = use_signal(String::new);
    let mut colours = use_signal(|| ["#000000ff"; 6].map(String::from));

    let mut update_colour = move |i: usize, c: HexColour| {
        colours.with_mut(|colours| colours[i] = c);
    };

    let handle_save = move |_| {
        if let Err(err) = Palette::validate_name(&name()) {
            toast.set(Some(Toast {
                message: err,
                toast_type: ToastType::Error,
            }));
            return;
        }
        on_save.call(Palette::custom(name(), colours()));

        name.set(String::new());
    };

    let preview = use_memo(move || {
        let display_name = if name().is_empty() {
            "Preview".to_string()
        } else {
            name()
        };
        Palette::custom(display_name, colours())
    });

    rsx! {
        div { class: "palette-editor",
            h3 { "Create Custom Palette" }

            div { class: "editor-section",
                label { "Preview" }
                PalettePreview { palette: preview() }
            }
        }

        div { class: "editor-section",
            label { "Palette Name" }
            input {
                r#type: "text",
                value: "{name}",
                placeholder: "My Custom Palette",
                oninput: move |event| name.set(event.value()),
            }
        }
        div { class: "editor-section",
            label { "Colours" }
            div { class: "colour-grid",
                for (i , colour) in colours().iter().enumerate() {
                    ColourPicker {
                        key: "{i}",
                        value: colour.clone(),
                        on_change: move |colour| update_colour(i, colour),
                    }
                }
            }
        }
        div { class: "editor-actions",
            button { onclick: handle_save, "Save Palette" }
            button { onclick: move |_| on_cancel.call(()), "Cancel" }
        }
    }
}
