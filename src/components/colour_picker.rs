use crate::hex_colour::{hex_colour_to_rgb, is_valid, normalize, HexColour};
use dioxus::prelude::*;

#[component]
pub fn ColourPicker(value: HexColour, on_change: EventHandler<HexColour>) -> Element {
    let rgb_only = hex_colour_to_rgb(&value);

    let mut mutable_value = use_signal(|| value.clone());

    let border_style = if is_valid(&mutable_value()) {
        "border: 1px solid #ccc;"
    } else {
        "border: 2px solid #f44336;"
    };

    rsx! {
        div { class: "colour-picker",
            input {
                aria_label: "Colour Picker",
                r#type: "color",
                value: "{rgb_only}",
                oninput: move |event| on_change.call(format!("{}ff", event.value())),
            }
            input {
                aria_label: "Hex Colour Input",
                r#type: "text",
                value: "{mutable_value}",
                style: "{border_style}",
                maxlength: 9,
                placeholder: "#rrggbbaa",
                oninput: move |event| {
                    let colour = normalize(&event.value());

                    mutable_value.set(colour.clone());

                    if is_valid(&colour) {
                        on_change.call(colour)
                    }
                },
            }
        }
    }
}
