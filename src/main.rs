// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use image::{open, GrayImage};

mod generator;
mod image_processor;

use crate::image_processor::{ImageBuffer, ImageProcessor, Kernel};

use crate::generator::{Generator, Plate, PlateConfig};

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
    // Plate code
    // let width: u32 = 600;
    // let height: u32 = 600;
    // let density: u32 = 10;
    // let dot_size: u32 = 2;

    // let plate_config = PlateConfig::new(width, height, density, dot_size);

    // let plate = Plate::new(plate_config);
    // let generator = Generator::new();

    // // let generated_plate = generator.paint(&mut plate.clone());

    // let SVG: String = generator.paint(plate);

    // rsx! {
    //     img { src: SVG.as_str() }
    // }

    let gray_image: GrayImage = image::open("guassian-test.png")?.to_luma8();
    let buffer = image_processor::ImageBuffer::from(&gray_image);

    let processed_image =
        image_processor::ImageProcessor::new(Box::new(image_processor::CrateImageProcessor))
            .gaussian_blur(&buffer, 3, 4.0);

    let _: Result<(), _> = processed_image.to_u8().to_file("blurred.png".as_ref());

    rsx! {
        img { src: "blurred.png" }
    }
}
