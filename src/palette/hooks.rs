use dioxus::prelude::*;

use super::Palette;

/// Hook to manage a persisted palette selection with automatic localStorage sync.
///
/// Stores the selected palette name in localStorage and restores it on load by
/// looking it up in the available_palettes list. This works for both preset and
/// custom palettes without coupling to storage implementation.
///
/// On web: Loads from localStorage and persists changes automatically.
/// On native: Returns the default palette (future: could load from config file).
pub fn use_persisted_palette(
    #[cfg(target_arch = "wasm32")] storage_key: &str,
    #[cfg(not(target_arch = "wasm32"))] _storage_key: &str,
    _available_palettes: Vec<Palette>,
    default: Palette,
) -> Signal<Palette> {
    let palette = use_signal(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    if let Ok(Some(name)) = local_storage.get_item(storage_key) {
                        // Find palette by name in available palettes
                        if let Some(found) = _available_palettes.iter().find(|p| p.name == name) {
                            return found.clone();
                        }
                    }
                }
            }
        }

        default
    });

    #[cfg(target_arch = "wasm32")]
    {
        let storage_key = storage_key.to_string();
        use_effect(move || {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    let _ = local_storage.set_item(&storage_key, &palette().name);
                }
            }
        });
    }

    palette
}
