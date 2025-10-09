use image::GrayImage;
use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Mul, Sub};
use super::kernel::Kernel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImageBuffer<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

pub trait FromF32Convolve: Sized {
    fn from_f32_convolve(val: f32) -> Self;
}

impl FromF32Convolve for u8 {
    fn from_f32_convolve(val: f32) -> Self {
        val.round().clamp(0.0, 255.0) as u8
    }
}
impl FromF32Convolve for f32 {
    fn from_f32_convolve(val: f32) -> Self {
        val
    }
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

impl<T> ImageBuffer<T>
where
    T: Copy + Into<f32> + FromF32Convolve,
{
    pub fn convolve<K>(&self, kernel: &Kernel<K>) -> ImageBuffer<T>
    where
        K: Copy + Into<f32>,
    {
        let mut convolved_data = Vec::with_capacity((self.width * self.height) as usize);

        // Out-of-bounds pixels are ignored (zero-padding)
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                let mut sum = 0.0;
                for ky in 0..kernel.size as isize {
                    for kx in 0..kernel.size as isize {
                        let ix = x + kx - (kernel.size as isize / 2);
                        let iy = y + ky - (kernel.size as isize / 2);
                        if ix >= 0
                            && ix < self.width as isize
                            && iy >= 0
                            && iy < self.height as isize
                        {
                            let pixel = self.get_pixel(ix as u32, iy as u32);
                            let kval = kernel.get(kx as usize, ky as usize);
                            sum += pixel.into() * kval.into();
                        }
                    }
                }

                convolved_data.push(T::from_f32_convolve(sum));
            }
        }

        ImageBuffer {
            data: convolved_data,
            width: self.width,
            height: self.height,
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

impl<T> Add for ImageBuffer<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl<T> Mul<T> for ImageBuffer<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|&v| v * rhs).collect();
        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl<T> Mul for ImageBuffer<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl<T> Sub for ImageBuffer<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

pub type GrayScaleBuffer = ImageBuffer<u8>;


#[cfg(test)]
mod tests {
    use super::*;



    use proptest::prelude::*;

    const EPSILON: f32 = 1e-5;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_convolve_identity_kernel() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];

        let image = ImageBuffer {
            data,
            width: 3,
            height: 3,
        };

        let identity_kernel = Kernel {
            size: 3,
            data: vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        };

        let result = image.convolve(&identity_kernel);

        assert_eq!(result.data, image.data);
    }

    #[test]
    fn test_convolve_box_blur_kernel() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        let image = ImageBuffer {
            data,
            width: 3,
            height: 3,
        };

        let box_blur_kernel = Kernel {
            size: 3,
            data: vec![1.0 / 9.0; 9],
        };

        let result = image.convolve(&box_blur_kernel);

        // Expected result calculated manually w/ zero-padding and clamped
        assert_eq!(result.data, [1, 2, 2, 3, 5, 4, 3, 4, 3]);
    }



    proptest! {
        #[test]
        fn test_convolve_linearity_property(
            ref data_a in prop::collection::vec(0.0..255.0f32, 9),
            ref data_b in prop::collection::vec(0.0..255.0f32, 9),
            scalar_a in 0.0..255.0f32,
            scalar_b in 0.0..255.0f32,
            ref kernel_data in prop::collection::vec(-5.0f32..=5.0, 9)
        ) {

            let width = 3;
            let height = 3;

            let image_a = ImageBuffer {
                data: data_a.clone(),
                width,
                height,
            };

            let image_b = ImageBuffer {
                data: data_b.clone(),
                width,
                height,
            };

            let kernel = Kernel {
                size: 3,
                data: kernel_data.clone(),
            };

            let image_ab = image_a.clone() * scalar_a + image_b.clone() * scalar_b;
            let convolved_ab = image_ab.convolve(&kernel);


            let convolved_a = image_a.convolve(&kernel);
            let convolved_b = image_b.convolve(&kernel);
            let expected = convolved_a * scalar_a + convolved_b * scalar_b;

            assert_eq!(convolved_ab, expected);
        }
    }
}
