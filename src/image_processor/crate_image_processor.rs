use crate::image_processor::{
    GrayScaleBuffer, ImageProcessorTrait, Kernel, SobelOrientation, NON_EDGE, STRONG_EDGE,
    WEAK_EDGE,
};

use crate::image_processor::image_buffer::{GradientBuffer, GradientComponent, GradientData};

use std::collections::VecDeque;

const STD_DEV: f32 = 3.0;
const KERNEL_SIZE: usize = 5;
const LOW_THRESHOLD: f32 = 0.05 * 255.0;
const HIGH_THRESHOLD: f32 = 0.50 * 255.0;

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

    fn compute_gradient(&self, image: &GrayScaleBuffer) -> GradientBuffer {
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
                let gx = gradient_x.get_pixel(x, y) as f32;
                let gy = gradient_y.get_pixel(x, y) as f32;

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

    fn non_maximum_suppression(&self, gradient_buffer: &GradientBuffer) -> GrayScaleBuffer {
        let mut suppressed_data: Vec<u8> =
            Vec::with_capacity((gradient_buffer.width * gradient_buffer.height) as usize);

        for y in 0..gradient_buffer.height {
            for x in 0..gradient_buffer.width {
                let current_gradient = gradient_buffer.get_pixel(x, y);
                let neighbours = current_gradient.discretize_direction();

                let mut is_local_maximum = true;

                for (dx, dy) in neighbours {
                    let neighbour_x = x as i32 + dx;
                    let neighbour_y = y as i32 + dy;

                    if neighbour_x >= 0
                        && neighbour_x < gradient_buffer.width as i32
                        && neighbour_y >= 0
                        && neighbour_y < gradient_buffer.height as i32
                    {
                        let neighbour_gradient =
                            gradient_buffer.get_pixel(neighbour_x as u32, neighbour_y as u32);

                        if neighbour_gradient.magnitude > current_gradient.magnitude {
                            is_local_maximum = false;
                            break;
                        }
                    }
                }

                if is_local_maximum {
                    suppressed_data.push(current_gradient.magnitude.clamp(0.0, 255.0) as u8);
                } else {
                    suppressed_data.push(0u8);
                }
            }
        }

        GrayScaleBuffer::new(
            gradient_buffer.width,
            gradient_buffer.height,
            suppressed_data,
        )
    }

    fn double_threshold(&self, image: &GrayScaleBuffer, low: f32, high: f32) -> GrayScaleBuffer {
        let data: Vec<u8> = image
            .data
            .iter()
            .map(|&val| {
                if (val as f32) >= high {
                    STRONG_EDGE
                } else if (val as f32) >= low {
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
        let edge_extended = self.edge_extension(&double_thresholded);

        return edge_extended;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image_processor::image_buffer::ApproxEq;
    use crate::image_processor::test_utils::TestDataStrategy;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_compute_gradient_magnitude_relationnship(buffer in GrayScaleBuffer::random()) {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);

            for &pixel in gradient_buffer.data.iter() {
                prop_assert!(pixel.magnitude <= pixel.x_component.abs() + pixel.y_component.abs());

                let expected_magnitude = (pixel.x_component.powi(2) + pixel.y_component.powi(2)).sqrt();
                prop_assert!(pixel.magnitude.approx_eq(&expected_magnitude));
            }

        }

        #[test]
        fn test_compute_gradient_zero_property(buffer in GrayScaleBuffer::uniform()) {
            let processor = CrateImageProcessor;
            let result = processor.compute_gradient(&buffer);

            for &pixel in result.data.iter() {
                prop_assert_eq!(pixel.magnitude, 0.0);
            }
        }

        #[test]
        fn test_double_threshold_classification(
            buffer in GrayScaleBuffer::random(),
            low in 0..254u8,
            high in 1..255u8,
        ) {
            prop_assume!(low < high);

            let processor = CrateImageProcessor;
            let result = processor.double_threshold(&buffer, low as f32, high as f32);

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
        fn test_non_maximum_suppression_thinning(buffer in GrayScaleBuffer::random()) {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);
            let suppressed = processor.non_maximum_suppression(&gradient_buffer);

            let gradient_as_u8: Vec<u8> = gradient_buffer.data.iter()
                .map(|g| g.magnitude.clamp(0.0, 255.0) as u8)
                .collect();

            let input_edge_pixels = gradient_as_u8.iter().filter(|&&p| p > 0).count();
            let output_edge_pixels = suppressed.data.iter().filter(|&&p| p > 0).count();

            prop_assert!(
                output_edge_pixels <= input_edge_pixels,
                "Non-maximum suppression increased edge count: {} input -> {} output",
                input_edge_pixels,
                output_edge_pixels
            );
        }

        #[test]
        fn test_non_maximum_suppression_zero_gradient(buffer in GrayScaleBuffer::uniform())
        {
            let processor = CrateImageProcessor;
            let gradient_buffer = processor.compute_gradient(&buffer);
            let suppressed = processor.non_maximum_suppression(&gradient_buffer);

            prop_assert!(suppressed.data.iter().all(|&p| p == 0));
        }
    }
}
