use std::collections::VecDeque;

use crate::image_processor::image_buffer::{GradientBuffer, GradientData};
use crate::image_processor::{
    GrayScaleBuffer, ImageBuffer, ImageProcessorTrait, Kernel, SobelOrientation, NON_EDGE,
    STRONG_EDGE, WEAK_EDGE,
};

const STD_DEV: f32 = 3.0;
const KERNEL_SIZE: usize = 19;
const LOW_THRESHOLD: f32 = 0.05 * 255.0;
const HIGH_THRESHOLD: f32 = 0.75 * 255.0;

pub struct CrateImageProcessor;

impl ImageProcessorTrait for CrateImageProcessor {
    fn gaussian_blur(
        &self,
        image: &GrayScaleBuffer,
        kernel_size: usize,
        std_dev: f32,
    ) -> ImageBuffer<f32> {
        let kernel = Kernel::<f32>::gaussian(kernel_size, std_dev);

        let f32_image: ImageBuffer<f32> = image.convert();

        f32_image.convolve(&kernel)
    }

    fn compute_gradient(&self, image: &ImageBuffer<f32>) -> GradientBuffer {
        let sobel_x = Kernel::<f32>::sobel(SobelOrientation::Horizontal);
        let sobel_y = Kernel::<f32>::sobel(SobelOrientation::Vertical);

        let gradient_x = image.convolve(&sobel_x);
        let gradient_y = image.convolve(&sobel_y);

        let mut gradient_buffer = GradientBuffer::new(
            image.width,
            image.height,
            vec![GradientData::default(); (image.width * image.height) as usize],
        );

        for y in 0..image.height {
            for x in 0..image.width {
                let gx = gradient_x.get_pixel(x, y);
                let gy = gradient_y.get_pixel(x, y);

                let magnitude = (gx.powi(2) + gy.powi(2)).sqrt();

                gradient_buffer.set_pixel(
                    x,
                    y,
                    GradientData {
                        magnitude,
                        x_component: gx,
                        y_component: gy,
                    },
                );
            }
        }

        gradient_buffer
    }

    fn non_maximum_suppression(&self, gradient_buffer: &GradientBuffer) -> ImageBuffer<f32> {
        let mut suppressed_data =
            vec![0.0f32; (gradient_buffer.width * gradient_buffer.height) as usize];

        for i in 1..gradient_buffer.height - 1 {
            for j in 1..gradient_buffer.width - 1 {
                let current_gradient = gradient_buffer.get_pixel(j, i);

                if current_gradient.x_component == 0.0 && current_gradient.y_component == 0.0 {
                    continue;
                }

                let norm = (current_gradient.x_component.powi(2)
                    + current_gradient.y_component.powi(2))
                .sqrt();

                let (dx, dy) = (
                    current_gradient.x_component / norm,
                    current_gradient.y_component / norm,
                );

                let mag1 = gradient_buffer.interpolate_magnitude(i as f32 + dy, j as f32 + dx);
                let mag2 = gradient_buffer.interpolate_magnitude(i as f32 - dy, j as f32 - dx);

                let index = (i * gradient_buffer.width + j) as usize;
                if current_gradient.magnitude >= mag1 && current_gradient.magnitude >= mag2 {
                    suppressed_data[index] = current_gradient.magnitude;
                }
            }
        }

        ImageBuffer::new(
            gradient_buffer.width,
            gradient_buffer.height,
            suppressed_data,
        )
    }

    fn double_threshold(&self, image: &ImageBuffer<f32>, low: f32, high: f32) -> GrayScaleBuffer {
        let data: Vec<u8> = image
            .data
            .iter()
            .map(|&val: &f32| {
                if val >= high {
                    STRONG_EDGE
                } else if val >= low {
                    WEAK_EDGE
                } else {
                    NON_EDGE
                }
            })
            .collect();

        GrayScaleBuffer::new(image.width, image.height, data)
    }

    fn edge_extension(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        let mut data = image.data.clone();
        let mut queue = VecDeque::new();

        for y in 0..image.height {
            for x in 0..image.width {
                let pixel_value = image.get_pixel(x, y);
                if pixel_value == STRONG_EDGE || pixel_value == WEAK_EDGE {
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let neighbour_x = x.saturating_add_signed(dx) as i32;
                    let neighbour_y = y.saturating_add_signed(dy) as i32;

                    // Check bounds using signed arithmetic
                    if neighbour_x >= 0
                        && neighbour_x < image.width as i32
                        && neighbour_y >= 0
                        && neighbour_y < image.height as i32
                    {
                        let neighbour_x = neighbour_x as u32;
                        let neighbour_y = neighbour_y as u32;
                        let index =
                            neighbour_y as usize * image.width as usize + neighbour_x as usize;

                        // If neighbour is a potential edge, promote it to strong edge
                        if data[index] == WEAK_EDGE {
                            data[index] = STRONG_EDGE;
                            queue.push_back((neighbour_x, neighbour_y));
                        }
                    }
                }
            }
        }

        // Any remaining weak edges that weren't connected to strong edges are discarded
        for pixel in data.iter_mut() {
            if *pixel == WEAK_EDGE {
                *pixel = NON_EDGE;
            }
        }

        GrayScaleBuffer::new(image.width, image.height, data)
    }

    fn canny_edge_detection(&self, image: &GrayScaleBuffer) -> GrayScaleBuffer {
        let blurred = self.gaussian_blur(image, KERNEL_SIZE, STD_DEV);
        let gradient_buffer = self.compute_gradient(&blurred);
        let non_max_suppressed = self.non_maximum_suppression(&gradient_buffer);
        let double_thresholded =
            self.double_threshold(&non_max_suppressed, LOW_THRESHOLD, HIGH_THRESHOLD);

        self.edge_extension(&double_thresholded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processor::image_buffer::ApproxEq;
    use crate::utils::test::TestDataStrategy;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_compute_gradient_magnitude_relationnship(buffer in ImageBuffer::<f32>::random()) {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);

            for &pixel in gradient_buffer.data.iter() {
                prop_assert!(pixel.magnitude <= pixel.x_component.abs() + pixel.y_component.abs());

                let expected_magnitude = (pixel.x_component.powi(2) + pixel.y_component.powi(2)).sqrt();
                prop_assert!(pixel.magnitude.approx_eq(&expected_magnitude));
            }

        }

        #[test]
        fn test_compute_gradient_zero_property(buffer in ImageBuffer::<f32>::uniform()) {
            let processor = CrateImageProcessor;
            let result = processor.compute_gradient(&buffer);

            for &pixel in result.data.iter() {
                prop_assert_eq!(pixel.magnitude, 0.0);
            }
        }

        #[test]
        fn test_double_threshold_classification(
            buffer in ImageBuffer::<f32>::random(),
            low in 0.0f32..254.0f32,
            high in 1.0f32..255.0f32,
        ) {
            prop_assume!(low < high);

            let processor = CrateImageProcessor;
            let result = processor.double_threshold(&buffer, low, high);

            for (input_val, &output_val) in buffer.data.iter().zip(result.data.iter()) {
                if *input_val >= high {
                    prop_assert_eq!(output_val, STRONG_EDGE);
                } else if *input_val >= low {
                    prop_assert_eq!(output_val, WEAK_EDGE);
                } else {
                    prop_assert_eq!(output_val, NON_EDGE);
                }
            }
        }

        #[test]
        fn test_edge_extension_eliminates_weak_edges(buffer in GrayScaleBuffer::random()) {
            let processor = CrateImageProcessor;
            let result = processor.edge_extension(&buffer);

            prop_assert!(result.data.iter().all(|&p| p != WEAK_EDGE));
        }

        #[test]
        fn test_edge_extension_preserves_strong_edges(buffer in GrayScaleBuffer::random()) {
            let processor = CrateImageProcessor;
            let result = processor.edge_extension(&buffer);

            for (input_val, &output_val) in buffer.data.iter().zip(result.data.iter()) {
                if *input_val == STRONG_EDGE {
                    prop_assert_eq!(output_val, STRONG_EDGE);
                }
            }
        }

        #[test]
        fn test_edge_extension_non_increasing(buffer in GrayScaleBuffer::random()) {
            let processor = CrateImageProcessor;

            let input_edges = buffer.data.iter().filter(|&&p| p == STRONG_EDGE || p == WEAK_EDGE).count();
            let result = processor.edge_extension(&buffer);
            let output_edges = result.data.iter().filter(|&&p| p == STRONG_EDGE).count();

            prop_assert!(output_edges <= input_edges);
        }

        #[test]
        fn test_non_maximum_suppression_thinning(buffer in ImageBuffer::<f32>::random()) {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);
            let suppressed = processor.non_maximum_suppression(&gradient_buffer);

            let gradient_as_u8: Vec<u8> = gradient_buffer.data.iter()
                .map(|g| g.magnitude.clamp(0.0, 255.0) as u8)
                .collect();

            let input_edge_pixels = gradient_as_u8.iter().filter(|&&p| p > 0).count();
            let output_edge_pixels = suppressed.data.iter().filter(|&&p| p > 0.0).count();

            prop_assert!(
                output_edge_pixels <= input_edge_pixels,
                "Non-maximum suppression increased edge count: {} input -> {} output",
                input_edge_pixels,
                output_edge_pixels
            );
        }

        #[test]
        fn test_non_maximum_suppression_zero_gradient(buffer in ImageBuffer::<f32>::uniform())
        {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);
            let suppressed = processor.non_maximum_suppression(&gradient_buffer);

            prop_assert!(suppressed.data.iter().all(|&p| p == 0.0));
        }
    }
}
