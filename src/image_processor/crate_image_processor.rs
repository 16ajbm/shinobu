use crate::image_processor::{GrayScaleBuffer, ImageProcessorTrait, Kernel, SobelOrientation};

pub struct CrateImageProcessor;

impl ImageProcessorTrait for CrateImageProcessor {
    fn gaussian_blur(
        &self,
        image: &GrayScaleBuffer,
        kernel_size: usize,
        std_dev: f32,
    ) -> GrayScaleBuffer {
        let kernel = Kernel::<f32>::gaussian(kernel_size, std_dev);
        image.convolve(&kernel)
    }

    fn compute_gradient(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        let sobel_x = Kernel::<f32>::sobel(SobelOrientation::Horizontal);
        let sobel_y = Kernel::<f32>::sobel(SobelOrientation::Vertical);

        let gradient_x = image.convolve(&sobel_x);
        let gradient_y = image.convolve(&sobel_y);

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

mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_compute_gradient_zero_property(a in 0..255u8) {

            let processor = CrateImageProcessor;

            let image = GrayScaleBuffer::new(3, 3, vec![a; 9]);


            let result = processor.compute_gradient(&image);

            for &pixel in result.data.iter() {
                prop_assert_eq!(pixel, 0);
            }
        }
    }
}
