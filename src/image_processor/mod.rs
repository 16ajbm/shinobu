pub mod clampable;
pub mod image_buffer;
pub mod image_processor;
pub mod kernel;
pub mod naive_image_processor;
pub mod sobel_orientation;

#[cfg(test)]
pub mod test_utils;

pub use clampable::Clampable;
pub use image_buffer::{GrayScaleBuffer, ImageBuffer};
pub use image_processor::{ImageProcessor, ImageProcessorTrait, NON_EDGE, STRONG_EDGE, WEAK_EDGE};
pub use kernel::Kernel;
pub use naive_image_processor::NaiveImageProcessor;
pub use sobel_orientation::SobelOrientation;
