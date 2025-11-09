// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use image::{open, GrayImage};

mod generator;
mod image_processor;

use crate::generator::poisson::PoissonSampler;
use crate::image_processor::{ImageBuffer, ImageProcessor, Kernel};

use crate::generator::{Generator, Palette, Plate, PlateConfig};

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
        let mut sampler = PoissonSampler::new(600.0, 600.0, 10.0, 30);
        let points = sampler.generate_samples();

        let config = PlateConfig {
            width: 600,
            height: 600,
            dot_size: 3,
            density: 0,           // Not used
            size_jitter: 0.2,     // Not used
            position_jitter: 0.0, // Not used
            primary: Palette::new("#000000".to_string()),
            secondary: Palette::new("#FFFFFF".to_string()),
            background: None, // Not used
        };

        let plate = Plate::from_points(points, config.clone());

        Generator::new().paint(plate)
    });

    rsx! {
        div { dangerous_inner_html: "{svg}" }
    }
}
