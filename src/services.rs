use crate::palette::Palette;

#[cfg(target_arch = "wasm32")]
mod platform_web;

#[cfg(not(target_arch = "wasm32"))]
mod platform_native;

/// Cross-platform export service
pub trait ExportService {
    fn export_palette(&self, palette: &Palette) -> Result<String, String>;
}

/// Cross-platform clipboard service  
pub trait ClipboardService {
    fn copy_text(&self, text: &str) -> Result<(), String>;
}

// Public constructors that hide platform details
pub fn create_export_service() -> Box<dyn ExportService> {
    #[cfg(target_arch = "wasm32")]
    return Box::new(platform_web::WebExportService);

    #[cfg(not(target_arch = "wasm32"))]
    return Box::new(platform_native::NativeExportService);
}

pub fn create_clipboard_service() -> Box<dyn ClipboardService> {
    #[cfg(target_arch = "wasm32")]
    return Box::new(platform_web::WebClipboardService);

    #[cfg(not(target_arch = "wasm32"))]
    return Box::new(platform_native::NativeClipboardService);
}

/// Service container for dependency injection
pub struct ServiceContainer {
    pub export_service: Box<dyn ExportService>,
    pub clipboard_service: Box<dyn ClipboardService>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            export_service: create_export_service(),
            clipboard_service: create_clipboard_service(),
        }
    }

    #[cfg(test)]
    pub fn new_mock(export_fails: bool, clipboard_fails: bool) -> Self {
        use crate::utils::test::{MockClipboardService, MockExportService};
        Self {
            export_service: Box::new(MockExportService::new(export_fails)),
            clipboard_service: Box::new(MockClipboardService::new(clipboard_fails)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::{MockClipboardService, MockExportService};

    #[test]
    fn test_export_service_success() {
        let service = MockExportService::new(false);
        let palette = Palette::red();

        let result = service.export_palette(&palette);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Mock exported: Red");
        assert_eq!(service.get_captured_palettes().len(), 1);
        assert_eq!(service.get_captured_palettes()[0].name, "Red");
    }

    #[test]
    fn test_export_service_failure() {
        let service = MockExportService::new(true);
        let palette = Palette::green();

        let result = service.export_palette(&palette);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mock export failure");
        // Should still capture the palette even on failure
        assert_eq!(service.get_captured_palettes().len(), 1);
    }

    #[test]
    fn test_clipboard_service_success() {
        let service = MockClipboardService::new(false);
        let text = "test clipboard content";

        let result = service.copy_text(text);

        assert!(result.is_ok());
        assert_eq!(service.get_captured_text(), vec![text]);
    }

    #[test]
    fn test_clipboard_service_failure() {
        let service = MockClipboardService::new(true);
        let text = "test content";

        let result = service.copy_text(text);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mock clipboard failure");
        // Should still capture the text even on failure
        assert_eq!(service.get_captured_text(), vec![text]);
    }

    #[test]
    fn test_service_container() {
        let container = ServiceContainer::new_mock(false, false);

        // Test that we can use the services through the container
        let palette = Palette::red();
        let result = container.export_service.export_palette(&palette);
        assert!(result.is_ok());

        let result = container.clipboard_service.copy_text("test");
        assert!(result.is_ok());
    }
}
