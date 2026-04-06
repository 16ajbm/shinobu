use dioxus::prelude::*;

use crate::config::PlateConfig;
use crate::generator::create_plate;

static SAD_ICON: Asset = asset!("/assets/sad-icon.svg");

#[component]
pub fn PlatePreview(plate_config: PlateConfig) -> Element {
    let plate: Result<String, String> = create_plate(
        plate_config.dot_size,
        plate_config.size_jitter,
        plate_config.spacing_multiplier,
        &plate_config.primary,
        &plate_config.secondary,
    );

    rsx! {
        div { class: "plate-preview",
            {
                match plate {
                    Ok(plate) => {
                        svg_content(crate::components::plate_preview::svg_contentProps {
                            plate: plate,
                        })
                    }
                    Err(error) => {
                        error_content(crate::components::plate_preview::error_contentProps {
                            error: error,
                        })
                    }
                }
            }
        }
    }
}

#[component]
fn svg_content(plate: String) -> Element {
    rsx! {
        div { class: "svg-container", dangerous_inner_html: "{plate}" }
    }
}

#[component]
fn error_content(error: String) -> Element {
    rsx! {
        div { class: "error-container",
            div { class: "sad-icon",
                img {
                    src: "{SAD_ICON}",
                    width: "60",
                    height: "60",
                    alt: "Sad icon",
                }
            }
            div { class: "error-message", {"Error generating plate: {error}"} }
        }
    }
}
