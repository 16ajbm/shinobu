use image::GrayImage;

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

pub struct Kernel {
    pub size: usize,    // e.g., 3 for a 3x3 kernel
    pub data: Vec<f32>, // flattened row-major order
}

impl Kernel {
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[y * self.size + x]
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, f32)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(move |(i, &v)| (i % self.size, i / self.size, v))
    }

    pub fn convolve<T: Copy + Into<f32> + From<u8>>(
        &self,
        image: &ImageBuffer<T>,
    ) -> ImageBuffer<T> {
        let mut convolved_data = vec![T::from(0); (image.width * image.height) as usize];

        for y in 0..image.height as isize {
            for x in 0..image.width as isize {
                let mut sum = 0.0;
                for ky in 0..self.size as isize {
                    for kx in 0..self.size as isize {
                        let ix = x + kx - (self.size as isize / 2);
                        let iy = y + ky - (self.size as isize / 2);
                        if ix >= 0
                            && ix < image.width as isize
                            && iy >= 0
                            && iy < image.height as isize
                        {
                            let pixel = image.get_pixel(ix as u32, iy as u32);
                            let kval = self.get(kx as usize, ky as usize);
                            sum += pixel.into() * kval;
                        }
                    }
                }

                let value = sum.round().clamp(0.0, 255.0) as u8;
                convolved_data[(y as usize) * (image.width as usize) + (x as usize)] =
                    T::from(value);
            }
        }

        ImageBuffer {
            data: convolved_data,
            width: image.width,
            height: image.height,
        }
    }

    pub fn gaussian(size: usize, std_dev: f32) -> Self {
        let mut data: Vec<f32> = Vec::with_capacity(size * size);

        for y in 0..size {
            for x in 0..size {
                let gx = x as isize - (size as isize / 2);
                let gy = y as isize - (size as isize / 2);
                let value = (1.0 / (2.0 * std::f32::consts::PI * std_dev * std_dev))
                    * (-((gx * gx + gy * gy) as f32) / (2.0 * std_dev * std_dev)).exp();
                data.push(value);
            }
        }

        // Normalize the kernel
        let sum: f32 = data.iter().sum();
        for v in &mut data {
            *v /= sum;
        }

        Self { size, data }
    }
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
        // Implement gradient computation logic here
        todo!()
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
