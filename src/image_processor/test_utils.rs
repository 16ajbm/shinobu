use super::*;
use crate::image_processor::image_buffer::GrayScaleBuffer;
use proptest::prelude::*;

pub trait TestDataStrategy<T> {
    fn uniform() -> BoxedStrategy<T>;
    fn random() -> BoxedStrategy<T>;
}

impl TestDataStrategy<GrayScaleBuffer> for GrayScaleBuffer {
    fn uniform() -> BoxedStrategy<GrayScaleBuffer> {
        (0u8..=255, 3u32..=10, 3u32..=10)
            .prop_flat_map(|(value, width, height)| {
                let size = (width * height) as usize;
                (
                    Just((width, height)),
                    prop::collection::vec(value..=value, size),
                )
            })
            .prop_map(|((width, height), data)| GrayScaleBuffer::new(width, height, data))
            .boxed()
    }

    fn random() -> BoxedStrategy<GrayScaleBuffer> {
        (3u32..=10, 3u32..=10)
            .prop_flat_map(|(width, height)| {
                let size = (width * height) as usize;
                (
                    Just((width, height)),
                    prop::collection::vec(0u8..=255, size),
                )
            })
            .prop_map(|((width, height), data)| GrayScaleBuffer::new(width, height, data))
            .boxed()
    }
}
