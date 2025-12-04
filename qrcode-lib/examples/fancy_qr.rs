// Example: Fancy QR code with custom styling
//
// This example demonstrates creating a stylish QR code with
// custom colors, shapes, and overlays.

use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};
use std::fs::File;
use std::io::Write;

fn main() {
    // Create fancy QR code
    let text = "https://example.com/";
    let qr = FancyQr::from_text(text)
        .expect("Failed to create QR code");
    
    // Customize rendering options
    let mut options = FancyOptions::default();
    
    // Set custom colors (purple theme)
    options.color_background = "#FAF5FF".to_string(); // Light purple background
    options.color_data = "#6B4B8A".to_string();       // Purple data
    options.color_finder = "#8B5CF6".to_string();     // Light purple finders
    
    // Use circular dots
    options.shape_module = ModuleShape::Circle;
    
    // Use rounded finder patterns
    options.shape_finder = FinderShape::Rounded(1.5);
    
    // Add center text
    options.center_text = Some("SCAN ME".to_string());
    options.overlay_scale = 0.2;
    
    // Generate SVG
    let svg = qr.render_svg(&options);
    
    // Save to file
    let filename = "fancy_qrcode.svg";
    let mut file = File::create(filename)
        .expect("Failed to create file");
    file.write_all(svg.as_bytes())
        .expect("Failed to write to file");
    
    println!("Fancy QR code saved to {}", filename);
}

