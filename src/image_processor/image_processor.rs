use crate::image_processor::{image_buffer::GradientBuffer, GrayScaleBuffer, ImageBuffer};

// Edge classification constants - available to all implementations
pub const NON_EDGE: u8 = 0;
pub const WEAK_EDGE: u8 = 127;
pub const STRONG_EDGE: u8 = 255;

pub struct ImageProcessor {
    pub adapter: Box<dyn ImageProcessorTrait>,
}

impl ImageProcessor {
    pub fn new(adapter: Box<dyn ImageProcessorTrait>) -> Self {
        Self { adapter }
    }

    pub fn gaussian_blur(
        &self,
        buffer: &ImageBuffer<u8>,
        kernel_size: usize,
        std_dev: f32,
    ) -> ImageBuffer<f32> {
        self.adapter.gaussian_blur(buffer, kernel_size, std_dev)
    }
}

pub trait ImageProcessorTrait {
    fn gaussian_blur(
        &self,
        image: &GrayScaleBuffer,
        kernel_size: usize,
        std_dev: f32,
    ) -> ImageBuffer<f32>;
    fn compute_gradient(&self, image: &ImageBuffer<f32>) -> GradientBuffer;
    fn non_maximum_suppression(&self, gradient_buffer: &GradientBuffer) -> ImageBuffer<f32>;
    fn double_threshold(&self, image: &ImageBuffer<f32>, low: f32, high: f32) -> GrayScaleBuffer;
    fn edge_extension(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
    fn canny_edge_detection(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
}
