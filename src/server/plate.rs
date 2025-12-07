#[allow(unused_imports)]
use crate::generator;
use crate::image_processor::ImageBuffer;
use dioxus::prelude::*;

#[server]
pub async fn generate_plate(
    dot_size: u32,
    size_jitter: f32,
    spacing_multiplier: f32,
) -> Result<String, ServerFnError> {
    use crate::palette::Palette;

    // Use default palettes for server-side generation
    let primary = Palette::red();
    let secondary = Palette::green();

    let svg = generator::create_plate(
        dot_size,
        size_jitter,
        spacing_multiplier,
        &primary,
        &secondary,
    )?;

    Ok(svg)
}

#[allow(dead_code)]
fn load_edge_map() -> Result<ImageBuffer<u8>, ServerFnError> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let image_path = format!("{}/extras/seven.png", manifest_dir);
    ImageBuffer::from_file(&image_path)
        .map_err(|e| ServerFnError::ServerError(format!("Failed to load image: {}", e)))
}
