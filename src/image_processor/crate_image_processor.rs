use crate::image_processor::{GrayScaleBuffer, ImageProcessorTrait, Kernel, SobelOrientation, NON_EDGE, WEAK_EDGE, STRONG_EDGE};

use crate::image_processor::image_buffer::{GradientBuffer, GradientData};

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
                    let neighbor_x = x as i32 + dx;
                    let neighbor_y = y as i32 + dy;

                    if neighbor_x >= 0
                        && neighbor_x < gradient_buffer.width as i32
                        && neighbor_y >= 0
                        && neighbor_y < gradient_buffer.height as i32
                    {
                        let neighbour_gradient =
                            gradient_buffer.get_pixel(neighbor_x as u32, neighbor_y as u32);

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

        image.data.iter().map(|&val| {
            if (val as f32) >= high {
                STRONG_EDGE
            } else if (val as f32) >= low {
                WEAK_EDGE
            } else {
                NON_EDGE
            }
        }).collect();
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
                prop_assert_eq!(pixel.magnitude, 0.0);
            }
        }
    }
}
