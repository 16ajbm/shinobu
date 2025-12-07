use dioxus::prelude::*;

use super::storage::PaletteStorage;
use super::Palette;

/// Hook to manage a persisted palette selection with automatic localStorage sync.
///
/// On web: Loads from localStorage and persists changes automatically.
/// On native: Returns the default palette (future: could load from config file).
pub fn use_persisted_palette<S: PaletteStorage + 'static>(
    #[cfg(target_arch = "wasm32")] storage_key: &str,
    #[cfg(not(target_arch = "wasm32"))] _storage_key: &str,
    storage: Signal<Option<S>>,
    default: Palette,
) -> Signal<Palette> {
    let palette = use_signal(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(local_storage)) = window.local_storage() {
                    if let Ok(Some(name)) = local_storage.get_item(storage_key) {
                        if let Some(ref store) = storage.read().as_ref() {
                            if let Ok(palette) = store.load(&name) {
                                return palette;
                            }
                        }
                    }
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        let _ = storage;

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
