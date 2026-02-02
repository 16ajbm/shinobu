use dioxus::prelude::*;

use crate::palette::Palette;

#[component]
pub fn PalettePreview(palette: Palette) -> Element {
    rsx! {
        div { style: "display: flex; gap: 2px; height: 24px; border-radius: 4px; overflow: hidden;",
            for color in palette.colors() {
                div { style: "flex: 1; background: {color}; min-width: 0;" }
            }
        }
    }
}
