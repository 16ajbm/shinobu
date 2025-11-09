pub mod clampable;
pub mod crate_image_processor;
pub mod image_buffer;
pub mod image_processor;
pub mod kernel;
pub mod sobel_orientation;

#[cfg(test)]
pub mod test_utils;

pub use image_buffer::{GrayScaleBuffer, ImageBuffer};
pub use image_processor::{ImageProcessor, ImageProcessorTrait, NON_EDGE, STRONG_EDGE, WEAK_EDGE};
pub use kernel::Kernel;
pub use sobel_orientation::SobelOrientation;
