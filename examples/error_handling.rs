// Example demonstrating the improved error handling system
use shinobu::palette::Palette;
use shinobu::services::{create_export_service, ExportError, ServiceError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let palette = Palette::red();
    let export_service = create_export_service();

    match export_service.export_palette(&palette) {
        Ok(message) => {
            println!("✅ Export successful: {}", message);
        }
        Err(ExportError::ExportFailed { palette_name, source }) => {
            println!("❌ Export failed for '{}': {}", palette_name, source);

            // Pattern match on the underlying service error for specific handling
            match source {
                ServiceError::Serialization(msg) => {
                    println!("🔧 This is a serialization issue: {}", msg);
                }
                ServiceError::Platform(msg) => {
                    println!("🖥️  This is a platform-specific issue: {}", msg);
                }
                ServiceError::Io(msg) => {
                    println!("💾 This is an I/O issue: {}", msg);
                }
                ServiceError::Unsupported(msg) => {
                    println!("⚠️  This feature is not supported: {}", msg);
                }
            }
        }
    }

    Ok(())
}