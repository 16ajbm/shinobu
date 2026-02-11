use dioxus::prelude::*;

use crate::palette::Palette;

use super::PalettePreview;

#[component]
pub fn PaletteEditor(on_save: EventHandler<Palette>) -> Element {
    let mut name = use_signal(String::new);
    let mut colors = use_signal(|| {
        [
            "#d32f2fff".to_string(),
            "#c62828ff".to_string(),
            "#e53935ff".to_string(),
            "#f44336ff".to_string(),
            "#ef5350ff".to_string(),
            "#e57373ff".to_string(),
        ]
    });
    let mut validation_error = use_signal(|| None::<String>);

    let current_palette = use_memo(move || Palette::custom("Preview".to_string(), colors()));

    let handle_save = move |_| {
        let name_value = name();

        // Validate name
        if let Err(err) = Palette::validate_name(&name_value) {
            validation_error.set(Some(err));
            return;
        }

        // Create and save palette
        let palette = Palette::custom(name_value, colors());
        on_save.call(palette);

        // Reset form
        name.set(String::new());
        validation_error.set(None);
    };

    rsx! {
        div {
            style: "padding: 16px; background: #f9f9f9; border-radius: 8px; margin-bottom: 16px;",

            h3 {
                style: "margin: 0 0 12px 0; font-size: 1.1em;",
                "Create Custom Palette"
            }

            // Live preview
            div {
                style: "margin-bottom: 12px;",
                p {
                    style: "font-size: 13px; margin-bottom: 6px; font-weight: 500;",
                    "Preview"
                }
                PalettePreview { palette: current_palette() }
            }

            // Name input
            div {
                style: "margin-bottom: 12px;",
                label {
                    style: "display: block; margin-bottom: 4px; font-size: 13px; font-weight: 500;",
                    "Palette Name"
                }
                input {
                    r#type: "text",
                    value: "{name}",
                    placeholder: "My Custom Palette",
                    style: "width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; font-size: 14px; box-sizing: border-box;",
                    oninput: move |evt| {
                        name.set(evt.value());
                        validation_error.set(None);
                    }
                }
                if let Some(error) = validation_error() {
                    p {
                        style: "color: #d32f2f; font-size: 12px; margin: 4px 0 0 0;",
                        "{error}"
                    }
                }
            }

            // Color pickers
            div {
                style: "margin-bottom: 12px;",
                p {
                    style: "font-size: 13px; margin-bottom: 8px; font-weight: 500;",
                    "Colors"
                }
                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px;",
                    for (i, color) in colors().iter().enumerate() {
                        div {
                            key: "{i}",
                            style: "display: flex; align-items: center; gap: 8px;",
                            input {
                                r#type: "color",
                                value: "{color.chars().take(7).collect::<String>()}", // Strip alpha for color input
                                style: "width: 40px; height: 40px; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;",
                                oninput: move |evt| {
                                    let new_color = format!("{}ff", evt.value()); // Add alpha
                                    let mut new_colors = colors();
                                    new_colors[i] = new_color;
                                    colors.set(new_colors);
                                }
                            }
                            input {
                                r#type: "text",
                                value: "{color}",
                                style: "flex: 1; padding: 6px; border: 1px solid #ccc; border-radius: 4px; font-size: 13px; font-family: monospace;",
                                maxlength: 9,
                                oninput: move |evt| {
                                    let mut new_colors = colors();
                                    new_colors[i] = evt.value();
                                    colors.set(new_colors);
                                }
                            }
                        }
                    }
                }
            }

            // Action buttons
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px;",
                button {
                    style: "padding: 10px; background: #4caf50; color: white; border: none; border-radius: 4px; font-size: 14px; font-weight: 600; cursor: pointer;",
                    onclick: handle_save,
                    "Save Palette"
                }
                button {
                    style: "padding: 10px; background: #2196f3; color: white; border: none; border-radius: 4px; font-size: 14px; font-weight: 600; cursor: pointer;",
                    onclick: move |_| {
                        let palette = Palette::custom("Export".to_string(), colors());
                        let json = serde_json::to_string_pretty(&palette).unwrap_or_default();

                        // Copy to clipboard (cross-platform service)
                        let clipboard_service = crate::services::create_clipboard_service();
                        let _ = clipboard_service.copy_text(&json);

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            // On native, just log it (could use system clipboard crate if desired)
                            println!("Exported palette JSON:\n{}", json);
                        }
                    },
                    "Export JSON"
                }
            }
        }
    }
}
