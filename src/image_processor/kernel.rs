use super::SobelOrientation;

#[derive(Debug, Clone)]
pub struct Kernel<T> {
    pub size: usize,  // e.g., 3 for a 3x3 kernel
    pub data: Vec<T>, // flattened row-major order
}

impl<T> Kernel<T> {
    pub fn new(size: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), size * size);
        Self { size, data }
    }

    pub fn get(&self, x: usize, y: usize) -> T
    where
        T: Copy,
    {
        self.data[y * self.size + x]
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, T)> + '_
    where
        T: Copy,
    {
        self.data
            .iter()
            .enumerate()
            .map(move |(i, &v)| (i % self.size, i / self.size, v))
    }

    pub fn gaussian(size: usize, std_dev: f32) -> Kernel<f32> {
        assert!(size % 2 == 1, "Kernel size must be odd");

        let mut data: Vec<f32> = Vec::with_capacity(size * size);

        let center = (size as isize) / 2;

        for y in 0..size {
            for x in 0..size {
                let gx = (x as isize) - center;
                let gy = (y as isize) - center;

                let distance_sq = (gx * gx + gy * gy) as f32;

                let std_dev_sq = std_dev * std_dev;

                let value = (1.0 / (2.0 * std::f32::consts::PI * std_dev_sq))
                    * (-distance_sq / (2.0 * std_dev_sq)).exp();

                data.push(value);
            }
        }

        let sum: f32 = data.iter().sum();

        data.iter_mut().for_each(|v| *v /= sum);

        Kernel { size, data }
    }

    pub fn sobel(orientation: SobelOrientation) -> Kernel<f32> {
        let (data, size) = match orientation {
            SobelOrientation::Horizontal => {
                (vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0], 3)
            }
            SobelOrientation::Vertical => (vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0], 3),
        };
        Kernel { size, data }
    }
}

impl<T> Kernel<T>
where
    T: Copy + std::ops::Mul<Output = T> + std::iter::Sum,
{
    pub fn dot_product(&self, other: &Kernel<T>) -> T {
        assert_eq!(
            self.size, other.size,
            "Kernels must be of the same size for dot product"
        );

        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
}

impl<T> Kernel<T>
where
    T: Copy + Into<f64> + Into<f32>,
{
    pub fn into_f64(&self) -> Kernel<f64> {
        let converted_data = self.data.iter().map(|&x| x.into()).collect();
        Kernel::new(self.size, converted_data)
    }

    pub fn into_f32(&self) -> Kernel<f32> {
        let converted_data = self.data.iter().map(|&x| x.into()).collect();
        Kernel::new(self.size, converted_data)
    }
}

impl<K> From<&Kernel<K>> for Vec<f64>
where
    K: Copy + Into<f64> + Into<f32>,
{
    fn from(kernel: &Kernel<K>) -> Vec<f64> {
        kernel.data.iter().map(|&x| x.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::image_processor::image_buffer::{ApproxEq, GrayScaleBuffer, ImageBuffer};
    use crate::utils::test::TestDataStrategy;

    use proptest::prelude::*;

    const EPSILON: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_sobel_kernel_orthogonality() {
        let sobel_x = Kernel::<f32>::sobel(SobelOrientation::Horizontal);
        let sobel_y = Kernel::<f32>::sobel(SobelOrientation::Vertical);

        let dot_product: f32 = sobel_x.dot_product(&sobel_y);

        assert!(
            approx_eq(dot_product, 0.0),
            "Dot product was {}, expected 0.0",
            dot_product
        );
    }

    proptest! {
        #[test]
        fn test_gaussian_kernel_normalization(
            size in 3u8..=11u8,
            std_dev in 0.1f32..10.0f32
        ) {
            let kernel_size = (size % 5) * 2 + 3; // Ensure it's odd and between 3 and 11

            let kernel = Kernel::<f32>::gaussian(kernel_size.into(), std_dev);

            let sum: f32 = kernel.data.iter().sum();

            assert!(approx_eq(sum, 1.0), "Kernel sum was {}, expected 1.0", sum);
        }

        #[test]
        fn test_sobel_horizontal_edge_response(horizontal_edge in GrayScaleBuffer::horizontal_edge()) {
            let sobel_x = Kernel::<f32>::sobel(SobelOrientation::Horizontal);
            let sobel_y = Kernel::<f32>::sobel(SobelOrientation::Vertical);

            let f32_horizontal_edge: ImageBuffer<f32> = horizontal_edge.convert();

            let gx_response = f32_horizontal_edge.convolve(&sobel_x);
            let gy_response = f32_horizontal_edge.convolve(&sobel_y);

            let gx_magnitude: f32 = gx_response.manhattan_norm() as f32;
            let gy_magnitude: f32 = gy_response.manhattan_norm() as f32;

            let total_gradient = gx_magnitude + gy_magnitude;
            let threshold = horizontal_edge.gradient_threshold();

            if total_gradient > threshold {
                prop_assert!(
                    gy_magnitude > gx_magnitude * 2.0,
                    "Horizontal edge: Y gradient ({:.2}) should be > X gradient ({:.2})",
                    gy_magnitude, gx_magnitude
                );
            }
        }

        #[test]
        fn test_sobel_preserves_strong_edges( edge_data in prop_oneof![
                GrayScaleBuffer::horizontal_edge().prop_map(|buf| (true, buf)),
                GrayScaleBuffer::vertical_edge().prop_map(|buf| (false, buf))
            ]) {
            let (is_horizontal, edge_buffer) = edge_data;

            let sobel = if is_horizontal {
                Kernel::<f32>::sobel(SobelOrientation::Vertical)
            } else {
                Kernel::<f32>::sobel(SobelOrientation::Horizontal)
            };

            let f32_buffer: ImageBuffer<f32> = edge_buffer.convert();

            let response = f32_buffer.convolve(&sobel);

            let contrast = edge_buffer.contrast();
            let magnitude = response.manhattan_norm();

            if contrast > 50 {
                prop_assert!(
                    magnitude > 0.0,
                    "Strong edge (contrast: {}) should produce non-zero gradient response (magnitude: {})",
                    contrast, magnitude
                );
            }
        }

        #[test]
        fn test_sobel_vertical_edge_response(vertical_edge in GrayScaleBuffer::vertical_edge()) {
            let sobel_x = Kernel::<f32>::sobel(SobelOrientation::Horizontal);
            let sobel_y = Kernel::<f32>::sobel(SobelOrientation::Vertical);

            let f32_vertical_edge: ImageBuffer<f32> = vertical_edge.convert();

            let gx_response = f32_vertical_edge.convolve(&sobel_x);
            let gy_response = f32_vertical_edge.convolve(&sobel_y);

            let gx_magnitude: f32 = gx_response.manhattan_norm() as f32;
            let gy_magnitude: f32 = gy_response.manhattan_norm() as f32;

            let total_gradient = gx_magnitude + gy_magnitude;
            let threshold = vertical_edge.gradient_threshold();

            if total_gradient > threshold {
                prop_assert!(
                    gx_magnitude > gy_magnitude * 2.0,
                    "Vertical edge: X gradient ({:.2}) should be > Y gradient ({:.2})",
                    gx_magnitude, gy_magnitude
                );
            }
        }
    }
}
