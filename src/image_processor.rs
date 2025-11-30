pub mod clampable;
pub mod crate_image_processor;
pub mod image_buffer;
pub mod image_processor;
pub mod kernel;
pub mod sobel_orientation;

pub use self::image_buffer::{GrayScaleBuffer, ImageBuffer};
pub use self::image_processor::{
    ImageProcessor, ImageProcessorTrait, NON_EDGE, STRONG_EDGE, WEAK_EDGE,
};
pub use self::kernel::Kernel;
pub use self::sobel_orientation::SobelOrientation;
