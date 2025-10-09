use crate::image_processor::{GrayScaleBuffer, ImageBuffer};

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
    ) -> ImageBuffer<u8> {
        self.adapter.gaussian_blur(buffer, kernel_size, std_dev)
    }
}

pub trait ImageProcessorTrait {
    fn gaussian_blur(
        &self,
        image: &GrayScaleBuffer,
        kernel_size: usize,
        std_dev: f32,
    ) -> GrayScaleBuffer;
    fn compute_gradient(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
    fn non_maximum_suppression(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
    fn double_threshold(&self, image: &GrayScaleBuffer, low: f32, high: f32) -> GrayScaleBuffer;
    fn edge_extension(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
    fn canny_edge_detection(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer;
}
