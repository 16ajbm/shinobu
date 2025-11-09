// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

mod generator;
mod image_processor;

use crate::generator::poisson::PoissonSampler;
use crate::image_processor::ImageBuffer;

use crate::generator::{Generator, Palette, PlateConfig};

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let svg = use_memo(move || {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        let image_path = format!("{}/extras/Serge.png", manifest_dir);

        let edge_map: ImageBuffer<u8> = match ImageBuffer::from_file(&image_path) {
            Ok(map) => {
                println!(
                    "Image: {}x{}, {} pixels",
                    map.width,
                    map.height,
                    map.data.len()
                );
                map
            }
            Err(e) => {
                println!("Failed to load image: {}", e);
                panic!("Cannot continue without image");
            }
        };

        let mut sampler =
            PoissonSampler::new(edge_map.width as f32, edge_map.height as f32, 5.0, 30);
        let points = sampler.generate_samples();

        println!("Generated {} points", points.len());

        let config = PlateConfig {
            width: edge_map.width,
            height: edge_map.height,
            dot_size: 3,
            density: 0,           // Not used
            size_jitter: 0.2,     // Not used
            position_jitter: 0.0, // Not used
            primary: Palette::red(),
            secondary: Palette::green(),
            background: None, // Not used
        };

        Generator::new().generate_and_paint(points, &edge_map, config)
    });

    rsx! {
        div { dangerous_inner_html: "{svg}" }
    }
}
