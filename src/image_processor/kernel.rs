use super::{ImageBuffer, SobelOrientation};

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

        // Out-of-bounds pixels are ignored (zero-padding)
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

    pub fn sobel(orientation: SobelOrientation) -> Self {
        let (data, size) = match orientation {
            SobelOrientation::Horizontal => {
                (vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], 3)
            }
            SobelOrientation::Vertical => (vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0], 3),
        };
        Self { size, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;

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

        let result = identity_kernel.convolve(&image);

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

        let result = box_blur_kernel.convolve(&image);

        // Expected result calculated manually w/ zero-padding and clamped
        assert_eq!(result.data, [1, 2, 2, 3, 5, 4, 3, 4, 3]);
    }

    #[test]
    fn test_gaussian_kernel_normalization() {
        let random_size: u8 = rand::random();
        let std_dev: f32 = rand::random();

        let kernel_size = (random_size % 5) * 2 + 3; // Ensure it's odd and between 3 and 11

        let kernel = Kernel::gaussian(kernel_size.into(), std_dev);

        let sum: f32 = kernel.data.iter().sum();

        assert!(approx_eq(sum, 1.0), "Kernel sum was {}, expected 1.0", sum);
    }
}
