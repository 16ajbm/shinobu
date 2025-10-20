pub mod clampable;
pub mod crate_image_processor;
pub mod image_buffer;
pub mod image_processor;
pub mod kernel;
pub mod sobel_orientation;

pub use clampable::Clampable;
pub use crate_image_processor::CrateImageProcessor;
pub use image_buffer::{GrayScaleBuffer, ImageBuffer};
pub use image_processor::{ImageProcessor, ImageProcessorTrait, NON_EDGE, WEAK_EDGE, STRONG_EDGE};
pub use kernel::Kernel;
pub use sobel_orientation::SobelOrientation;
