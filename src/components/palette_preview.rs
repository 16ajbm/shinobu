use dioxus::prelude::*;

use crate::palette::Palette;

#[component]
pub fn PalettePreview(palette: Palette) -> Element {
    rsx! {
        div { style: "display: flex; gap: 2px; height: 24px; border-radius: 4px; overflow: hidden;",
            for colour in palette.colours() {
                div { style: "flex: 1; background: {colour}; min-width: 0;" }
            }
        }
    }
}
