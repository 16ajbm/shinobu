pub mod kernel;

use image::GrayImage;

pub use kernel::Kernel;

pub trait Clampable: Copy {
    fn clamp_from_f32(val: f32) -> Self;
}

impl Clampable for u8 {
    fn clamp_from_f32(val: f32) -> Self {
        val.round().clamp(0.0, 255.0) as u8
    }
}

impl Clampable for u16 {
    fn clamp_from_f32(val: f32) -> Self {
        val.round().clamp(0.0, 65535.0) as u16
    }
}

impl Clampable for f32 {
    fn clamp_from_f32(val: f32) -> Self {
        val.clamp(0.0, 1.0)
    }
}

enum SobelOrientation {
    Horizontal,
    Vertical,
}

pub struct ImageBuffer<T> {
    data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T> ImageBuffer<T> {
    fn get_pixel(&self, x: u32, y: u32) -> T
    where
        T: Copy,
    {
        self.data[(y * self.width + x) as usize]
    }

    pub fn new(width: u32, height: u32, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
}

impl From<&GrayImage> for ImageBuffer<u8> {
    fn from(img: &GrayImage) -> Self {
        Self {
            width: img.width(),
            height: img.height(),
            data: img.as_raw().clone(),
        }
    }
}

// Convert from ImageBuffer to GrayImage
impl From<&ImageBuffer<u8>> for GrayImage {
    fn from(buf: &ImageBuffer<u8>) -> Self {
        GrayImage::from_raw(buf.width, buf.height, buf.data.clone()).expect("Invalid buffer size")
    }
}

type GrayScaleBuffer = ImageBuffer<u8>;

pub struct ImageProcessor {
    adapter: Box<dyn ImageProcessorTrait>,
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

pub struct CrateImageProcessor;

impl ImageProcessorTrait for CrateImageProcessor {
    fn gaussian_blur(
        &self,
        image: &GrayScaleBuffer,
        kernel_size: usize,
        std_dev: f32,
    ) -> GrayScaleBuffer {
        let kernel = Kernel::gaussian(kernel_size, std_dev);
        kernel.convolve(image)
    }

    fn compute_gradient(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        let sobel_x = Kernel::sobel(SobelOrientation::Horizontal);
        let sobel_y = Kernel::sobel(SobelOrientation::Vertical);

        let gradient_x = sobel_x.convolve(image);
        let gradient_y = sobel_y.convolve(image);

        let mut gradient_image = vec![0u8; (image.width * image.height) as usize];

        for y in 0..image.height {
            for x in 0..image.width {
                let gx = gradient_x.get_pixel(x, y) as f32;
                let gy = gradient_y.get_pixel(x, y) as f32;
                let magnitude = (gx.powi(2) + gy.powi(2)).sqrt().clamp(0.0, 255.0) as u8;
                gradient_image[(y * image.width + x) as usize] = magnitude;
            }
        }

        GrayScaleBuffer::new(image.width, image.height, gradient_image)
    }

    fn non_maximum_suppression(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        // Implement non-maximum suppression logic here
        todo!()
    }

    fn double_threshold(&self, image: &GrayScaleBuffer, low: f32, high: f32) -> GrayScaleBuffer {
        // Implement double threshold logic here
        todo!()
    }

    fn edge_extension(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        // Implement edge extension logic here
        todo!()
    }

    fn canny_edge_detection(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        // Compose the steps here
        todo!()
    }
}

// Future implementation
// pub struct ShaderImageProcessor;
