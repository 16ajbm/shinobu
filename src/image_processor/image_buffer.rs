use image::GrayImage;

pub struct ImageBuffer<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T> ImageBuffer<T> {
    pub fn get_pixel(&self, x: u32, y: u32) -> T
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

impl From<&ImageBuffer<u8>> for GrayImage {
    fn from(buf: &ImageBuffer<u8>) -> Self {
        GrayImage::from_raw(buf.width, buf.height, buf.data.clone()).expect("Invalid buffer size")
    }
}

pub type GrayScaleBuffer = ImageBuffer<u8>;
