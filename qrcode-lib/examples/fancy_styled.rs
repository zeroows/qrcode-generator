// Example: Multiple fancy QR code styles
//
// This example generates several QR codes with different visual styles.

use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};
use std::fs::{self, File};
use std::io::Write;

fn main() {
    // Create output directory
    fs::create_dir_all("output").expect("Failed to create output directory");
    
    let url = "https://example.com/";
    
    // Style 1: Classic (default)
    println!("Generating classic style...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let svg = qr.render_svg_default();
    save_svg("output/qr_classic.svg", &svg);
    
    // Style 2: Rounded with purple theme
    println!("Generating rounded style...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    options.color_background = "#FAF5FF".to_string(); // Light purple
    options.color_data = "#6B4B8A".to_string();       // Purple
    options.color_finder = "#8B5CF6".to_string();     // Light purple
    options.shape_module = ModuleShape::RoundedSquare(0.3);
    options.shape_finder = FinderShape::Rounded(1.0);
    let svg = qr.render_svg(&options);
    save_svg("output/qr_rounded.svg", &svg);
    
    // Style 3: Dots with text overlay
    println!("Generating dots style with overlay...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    options.color_background = "#F5F3FF".to_string();  // Very light purple
    options.color_data = "#7C3AED".to_string();        // Purple
    options.color_finder = "#A78BFA".to_string();      // Light purple
    options.shape_module = ModuleShape::Circle;
    options.shape_finder = FinderShape::Rounded(1.5);
    options.center_text = Some("SCAN".to_string());
    options.overlay_scale = 0.25;
    let svg = qr.render_svg(&options);
    save_svg("output/qr_dots_overlay.svg", &svg);
    
    // Style 4: Minimal monochrome
    println!("Generating minimal style...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    options.color_background = "#FFFFFF".to_string();
    options.color_data = "#000000".to_string();
    options.color_finder = "#000000".to_string();
    options.shape_module = ModuleShape::Square;
    options.shape_finder = FinderShape::Square;
    let svg = qr.render_svg(&options);
    save_svg("output/qr_minimal.svg", &svg);
    
    // Style 5: Modern with heavy rounding (gradient-inspired)
    println!("Generating modern style...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    options.color_background = "#FAF5FF".to_string();  // Light purple
    options.color_data = "#5B4B8A".to_string();        // Deep purple
    options.color_finder = "#7C3AED".to_string();      // Medium purple
    options.shape_module = ModuleShape::RoundedSquare(0.4);
    options.shape_finder = FinderShape::Rounded(2.0);
    let svg = qr.render_svg(&options);
    save_svg("output/qr_modern.svg", &svg);
    
    println!("\nAll QR codes generated in ./output/ directory!");
}

fn save_svg(filename: &str, svg: &str) {
    let mut file = File::create(filename)
        .expect("Failed to create file");
    file.write_all(svg.as_bytes())
        .expect("Failed to write to file");
    println!("  Saved: {}", filename);
}

