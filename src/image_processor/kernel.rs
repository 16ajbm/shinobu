use super::{ImageBuffer, SobelOrientation};

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

    use proptest::prelude::*;

    const EPSILON: f32 = 1e-5;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
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
    }
}
