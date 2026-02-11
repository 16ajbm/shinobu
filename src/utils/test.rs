use proptest::prelude::*;

use crate::image_processor::{GrayScaleBuffer, ImageBuffer};

#[cfg(test)]
use crate::palette::Palette;
#[cfg(test)]
use crate::services::{ClipboardError, ClipboardService, ExportError, ExportService, ServiceError};

pub trait TestDataStrategy<T> {
    fn uniform() -> BoxedStrategy<T>;
    fn random() -> BoxedStrategy<T>;
    fn horizontal_edge() -> BoxedStrategy<T>;
    fn vertical_edge() -> BoxedStrategy<T>;
}

impl TestDataStrategy<ImageBuffer<f32>> for ImageBuffer<f32> {
    fn uniform() -> BoxedStrategy<ImageBuffer<f32>> {
        GrayScaleBuffer::uniform()
            .prop_map(|gray| gray.convert())
            .boxed()
    }

    fn random() -> BoxedStrategy<ImageBuffer<f32>> {
        GrayScaleBuffer::random()
            .prop_map(|gray| gray.convert())
            .boxed()
    }

    fn horizontal_edge() -> BoxedStrategy<ImageBuffer<f32>> {
        GrayScaleBuffer::horizontal_edge()
            .prop_map(|gray| gray.convert())
            .boxed()
    }

    fn vertical_edge() -> BoxedStrategy<ImageBuffer<f32>> {
        GrayScaleBuffer::vertical_edge()
            .prop_map(|gray| gray.convert())
            .boxed()
    }
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

    fn horizontal_edge() -> BoxedStrategy<GrayScaleBuffer> {
        (0u8..=128, 127u8..=255, 4u32..=10, 4u32..=10, 1u32..10)
            .prop_map(|(low, high, width, height, edge_offset)| {
                let edge_row = 1 + (edge_offset % (height - 2));
                let mut data = vec![low; (width * height) as usize];

                for y in edge_row..height {
                    for x in 0..width {
                        data[(y * width + x) as usize] = high;
                    }
                }

                GrayScaleBuffer::new(width, height, data)
            })
            .boxed()
    }

    fn vertical_edge() -> BoxedStrategy<GrayScaleBuffer> {
        (0u8..=128, 127u8..=255, 4u32..=10, 4u32..=10, 1u32..10)
            .prop_map(|(low, high, width, height, edge_offset)| {
                let edge_col = 1 + (edge_offset % (width - 2));
                let mut data = vec![low; (width * height) as usize];

                for y in 0..height {
                    for x in edge_col..width {
                        data[(y * width + x) as usize] = high;
                    }
                }

                GrayScaleBuffer::new(width, height, data)
            })
            .boxed()
    }
}

/// Mock export service for testing
#[cfg(test)]
pub struct MockExportService {
    pub should_fail: bool,
    pub captured_palettes: std::cell::RefCell<Vec<Palette>>,
}

#[cfg(test)]
impl MockExportService {
    pub fn new(should_fail: bool) -> Self {
        Self {
            should_fail,
            captured_palettes: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn get_captured_palettes(&self) -> Vec<Palette> {
        self.captured_palettes.borrow().clone()
    }
}

#[cfg(test)]
impl ExportService for MockExportService {
    fn export_palette(&self, palette: &Palette) -> Result<String, ExportError> {
        self.captured_palettes.borrow_mut().push(palette.clone());

        if self.should_fail {
            Err(ExportError::ExportFailed {
                palette_name: palette.name.clone(),
                source: ServiceError::Platform("Mock export failure".to_string()),
            })
        } else {
            Ok(format!("Mock exported: {}", palette.name))
        }
    }
}

/// Mock clipboard service for testing
#[cfg(test)]
pub struct MockClipboardService {
    pub should_fail: bool,
    pub captured_text: std::cell::RefCell<Vec<String>>,
}

#[cfg(test)]
impl MockClipboardService {
    pub fn new(should_fail: bool) -> Self {
        Self {
            should_fail,
            captured_text: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn get_captured_text(&self) -> Vec<String> {
        self.captured_text.borrow().clone()
    }
}

#[cfg(test)]
impl ClipboardService for MockClipboardService {
    fn copy_text(&self, text: &str) -> Result<(), ClipboardError> {
        self.captured_text.borrow_mut().push(text.to_string());

        if self.should_fail {
            Err(ClipboardError::CopyFailed {
                source: ServiceError::Platform("Mock clipboard failure".to_string()),
            })
        } else {
            Ok(())
        }
    }
}
