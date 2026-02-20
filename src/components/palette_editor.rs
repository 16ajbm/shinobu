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
    let mut colours = use_signal(["#000000ff"; 6].map(String::from));

    let update_colour = move |i: usize, c: HexColour| {
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

    rsx! {
        div { class: "palette-editor",
            h3 { "Create Custom Palette" }
        }
    }
}
