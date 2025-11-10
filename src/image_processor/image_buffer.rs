use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Mul, Sub};
use std::path::Path;

use image::GrayImage;

use super::kernel::Kernel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImageBuffer<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

pub trait ToU8 {
    fn to_u8(self) -> u8;
}

impl ToU8 for u8 {
    fn to_u8(self) -> u8 {
        self // Identity conversion
    }
}

impl ToU8 for f32 {
    fn to_u8(self) -> u8 {
        self.clamp(0.0, 255.0).round() as u8
    }
}

// Now add a generic conversion method
impl<T> ImageBuffer<T>
where
    T: Copy + ToU8,
{
    pub fn to_u8(&self) -> ImageBuffer<u8> {
        ImageBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|&x| x.to_u8()).collect(),
        }
    }
}

pub trait FromConvolve<F>: Sized {
    fn from_convolve(val: F) -> Self;
}

// Then implement for different combinations:
impl FromConvolve<f32> for u8 {
    fn from_convolve(val: f32) -> Self {
        val.round().clamp(0.0, 255.0) as u8
    }
}

impl FromConvolve<f64> for u8 {
    fn from_convolve(val: f64) -> Self {
        val.round().clamp(0.0, 255.0) as u8
    }
}

impl FromConvolve<f32> for f32 {
    fn from_convolve(val: f32) -> Self {
        val
    }
}

impl FromConvolve<f64> for f32 {
    fn from_convolve(val: f64) -> Self {
        val as f32
    }
}

pub enum GradientComponent {
    Magnitude,
    XComponent,
    YComponent,
}

impl<T> ImageBuffer<T> {
    pub fn get_pixel(&self, x: u32, y: u32) -> T
    where
        T: Copy,
    {
        self.data[(y * self.width + x) as usize]
    }

    pub fn euclidean_norm(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        self.data
            .iter()
            .map(|&v| {
                let val: f64 = v.into();
                val * val
            })
            .sum::<f64>()
            .sqrt()
    }

    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: FromConvolve<f32> + Copy,
    {
        let img = image::open(file)?;

        let gray_img = img.to_luma8();

        let (width, height) = gray_img.dimensions();

        Ok(Self {
            width,
            height,
            data: gray_img
                .as_raw()
                .iter()
                .map(|&v| T::from_convolve(v as f32))
                .collect(),
        })
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>>
    where
        T: FromConvolve<f32> + Copy,
    {
        let img = image::load_from_memory(bytes)?;

        let gray_img = img.to_luma8();

        let (width, height) = gray_img.dimensions();

        Ok(Self {
            width,
            height,
            data: gray_img
                .as_raw()
                .iter()
                .map(|&v| T::from_convolve(v as f32))
                .collect(),
        })
    }

    pub fn manhattan_norm(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        self.data.iter().map(|&v| v.into().abs()).sum()
    }

    pub fn new(width: u32, height: u32, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: T) {
        self.data[(y * self.width + x) as usize] = value;
    }

    pub fn to_file(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Into<u8> + Copy,
    {
        let gray_image = GrayImage::from_raw(
            self.width,
            self.height,
            self.data.iter().map(|&v| v.into()).collect(),
        )
        .ok_or("Failed to create GrayImage from buffer")?;

        gray_image.save(path)?;

        Ok(())
    }
}

impl GradientBuffer {
    pub fn to_file_with_component(
        &self,
        path: &Path,
        component: GradientComponent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let gray_image = GrayImage::from_raw(
            self.width,
            self.height,
            self.data
                .iter()
                .map(|&v| match component {
                    GradientComponent::Magnitude => v.magnitude as u8,
                    GradientComponent::XComponent => v.x_component as u8,
                    GradientComponent::YComponent => v.y_component as u8,
                })
                .collect(),
        )
        .ok_or("Failed to create GrayImage from buffer")?;

        gray_image.save(path)?;

        Ok(())
    }

    pub fn interpolate_magnitude(&self, y: f32, x: f32) -> f32 {
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);

        let a = x - x0 as f32;
        let b = y - y0 as f32;

        let top_left = self.get_pixel(x0, y0).magnitude;
        let top_right = self.get_pixel(x1, y0).magnitude;
        let bottom_left = self.get_pixel(x0, y1).magnitude;
        let bottom_right = self.get_pixel(x1, y1).magnitude;

        top_left * (1.0 - a) * (1.0 - b)
            + top_right * a * (1.0 - b)
            + bottom_left * (1.0 - a) * b
            + bottom_right * a * b
    }
}

impl<T> ImageBuffer<T>
where
    T: Copy + Into<f32> + Ord + Sub<Output = T>,
{
    pub fn contrast(&self) -> T {
        let min_val = *self.data.iter().min().unwrap();
        let max_val = *self.data.iter().max().unwrap();
        max_val - min_val
    }

    pub fn gradient_threshold(&self) -> f32 {
        self.contrast().into() * 0.1
    }
}

impl<T> ImageBuffer<T>
where
    T: Copy,
{
    pub fn convert<U>(&self) -> ImageBuffer<U>
    where
        U: Copy,
        T: Into<U>,
    {
        ImageBuffer {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|&x| x.into()).collect(),
        }
    }
}

impl<T> ImageBuffer<T>
where
    T: Copy + Into<f32> + FromConvolve<f64>,
{
    pub fn convolve<K>(&self, kernel: &Kernel<K>) -> ImageBuffer<T>
    where
        K: Copy + Into<f32> + Into<f64>,
    {
        let mut convolved_data = Vec::with_capacity((self.width * self.height) as usize);

        let kernel_f64 = kernel.into_f64();

        // Use constant padding (replicate edge values)
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                // Kahan summation variables (per pixel)
                let mut sum: f64 = 0.0;
                let mut compensation: f64 = 0.0;

                for ky in 0..kernel.size as isize {
                    for kx in 0..kernel.size as isize {
                        let ix = x + kx - (kernel.size as isize / 2);
                        let iy = y + ky - (kernel.size as isize / 2);

                        // Clamp coordinates to image bounds (constant padding)
                        let clamped_x = ix.clamp(0, self.width as isize - 1) as u32;
                        let clamped_y = iy.clamp(0, self.height as isize - 1) as u32;

                        let pixel = self.get_pixel(clamped_x, clamped_y);
                        let kval = kernel_f64.get(kx as usize, ky as usize);

                        // Kahan summation
                        let term = (pixel.into() as f64) * kval - compensation;
                        let new_sum = sum + term;
                        compensation = (new_sum - sum) - term;
                        sum = new_sum;
                    }
                }

                convolved_data.push(T::from_convolve(sum));
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

#[derive(Debug, Clone, Copy, Default)]
pub struct GradientData<T> {
    pub magnitude: T,
    pub x_component: T,
    pub y_component: T,
}

impl<T> GradientData<T> {
    pub fn new(magnitude: T, x_component: T, y_component: T) -> Self {
        Self {
            magnitude,
            x_component,
            y_component,
        }
    }
}

impl GradientData<f32> {
    pub fn angle(&self) -> f32 {
        self.y_component.atan2(self.x_component)
    }

    pub fn discretize_direction(&self) -> [(i32, i32); 2] {
        let angle_rad = self.angle();

        // Normalize angle to [0, 180) degrees for undirected edges
        let angle_deg = (angle_rad.to_degrees() + 180.0) % 180.0;

        let bin = (angle_deg / 45.0).floor() as i32;

        match bin {
            0 => [(-1, 0), (1, 0)],  // Horizontal: left, right
            1 => [(-1, -1), (1, 1)], // Diagonal 45°: NW, SE
            2 => [(0, -1), (0, 1)],  // Vertical: up, down
            3 => [(-1, 1), (1, -1)], // Diagonal 135°: SW, NE
            _ => unreachable!(),
        }
    }
}

impl GradientData<f64> {
    pub fn angle(&self) -> f64 {
        self.y_component.atan2(self.x_component)
    }
}

pub type GradientBuffer = ImageBuffer<GradientData<f32>>;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(&self, other: &Rhs) -> bool;
}

impl ApproxEq for u8 {
    fn approx_eq(&self, other: &u8) -> bool {
        self == other
    }
}

impl ApproxEq for i32 {
    fn approx_eq(&self, other: &i32) -> bool {
        self == other
    }
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &f32) -> bool {
        const ABS_EPSILON: f32 = 1e-3;
        const REL_EPSILON: f32 = 1e-3;

        let diff = (self - other).abs();

        // Absolute epsilon check for small numbers
        if diff <= ABS_EPSILON {
            return true;
        }

        // Relative epsilon check for larger numbers
        let max_val = self.abs().max(other.abs());
        if max_val > 0.0 {
            diff / max_val <= REL_EPSILON
        } else {
            true
        }
    }
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &f64) -> bool {
        const ABS_EPSILON: f64 = 1e-3;
        const REL_EPSILON: f64 = 1e-3;

        let diff = (self - other).abs();

        // For small numbers, use absolute epsilon
        if diff <= ABS_EPSILON {
            return true;
        }

        // For larger numbers, use relative epsilon
        let max_val = self.abs().max(other.abs());
        if max_val > 0.0 {
            diff / max_val <= REL_EPSILON
        } else {
            true
        }
    }
}

impl<T: ApproxEq> ApproxEq for ImageBuffer<T> {
    fn approx_eq(&self, other: &ImageBuffer<T>) -> bool {
        self.width == other.width
            && self.height == other.height
            && self.data.len() == other.data.len()
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .all(|(a, b)| a.approx_eq(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_convolve_identity_kernel(
            data in prop::collection::vec(0u8..=255, 9)
        ) {
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

            assert!(result.approx_eq(&image));
       }

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

            assert!(convolved_ab.approx_eq(&expected));
        }
    }
}
