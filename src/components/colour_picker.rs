use crate::hex_colour::{hex_colour_to_rgb, HexColour};
use dioxus::prelude::*;

#[component]
pub fn ColourPicker(value: HexColour, on_change: EventHandler<HexColour>) -> Element {
    let rgb_only = hex_colour_to_rgb(&value);

    rsx! {
        div { class: "colour-picker",
            input {
                r#type: "color",
                value: "{rgb_only}",
                oninput: move |event| on_change.call(format!("{}ff", event.value())),
            }
            input {
                r#type: "text",
                value: "{value}",
                maxlength: 9,
                placeholder: "#rrggbbaa",
                oninput: move |event| on_change.call(event.value()),
            }
        }
    }
}
