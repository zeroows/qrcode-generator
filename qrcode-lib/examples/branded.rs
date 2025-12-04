// Example: Branded QR code with logo overlay
//
// This example demonstrates creating a branded QR code with a logo
// in the center, using custom brand colors.

use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};
use std::fs::{self, File};
use std::io::Write;

fn main() {
    println!("Generating branded QR codes...\n");
    
    // Create output directory
    fs::create_dir_all("output").expect("Failed to create output directory");
    
    let url = "https://example.com/";
    
    // Style 1: With embedded logo (using data URI)
    println!("1. Generating QR with embedded logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Brand colors
    options.color_background = "#FFFFFF".to_string();
    options.color_data = "#4d3695".to_string();        // Brand purple
    options.color_finder = "#4d3695".to_string();       // Brand purple
    
    // Rounded, modern look
    options.shape_module = ModuleShape::RoundedSquare(0.3);
    options.shape_finder = FinderShape::Rounded(1.5);
    
    // Embed the logo using a file path reference
    // Note: In production, you'd want to convert this to a data URI or use an absolute URL
    options.center_image_url = Some("logo-icon.svg".to_string());
    options.overlay_scale = 0.3; // 30% size for logo visibility
    
    let svg = qr.render_svg(&options);
    save_svg("output/with_logo.svg", &svg);
    
    // Style 2: With base64 embedded logo
    println!("2. Generating QR with base64 embedded logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    options.color_background = "#F8F7FF".to_string();  // Very light purple
    options.color_data = "#4d3695".to_string();
    options.color_finder = "#6B4B8A".to_string();
    
    options.shape_module = ModuleShape::Circle;
    options.shape_finder = FinderShape::Rounded(2.0);
    
    // Read and embed logo as base64 data URI
    let logo_svg = fs::read_to_string("logo-icon.svg")
        .expect("Failed to read logo file");
    let logo_base64 = base64_encode_svg(&logo_svg);
    options.center_image_url = Some(logo_base64);
    options.overlay_scale = 0.28;
    
    let svg = qr.render_svg(&options);
    save_svg("output/with_logo_base64.svg", &svg);
    
    // Style 3: Minimal style with logo
    println!("3. Generating minimal style with logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    options.color_background = "#FFFFFF".to_string();
    options.color_data = "#000000".to_string();
    options.color_finder = "#4d3695".to_string();      // Purple finders for brand recognition
    
    options.shape_module = ModuleShape::Square;
    options.shape_finder = FinderShape::Rounded(1.0);
    
    // Read logo
    let logo_svg = fs::read_to_string("logo-icon.svg")
        .expect("Failed to read logo file");
    let logo_base64 = base64_encode_svg(&logo_svg);
    options.center_image_url = Some(logo_base64);
    options.overlay_scale = 0.25;
    
    let svg = qr.render_svg(&options);
    save_svg("output/minimal_logo.svg", &svg);
    
    // Style 4: Gradient-inspired with logo (Premium look)
    println!("4. Generating gradient-inspired style with logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Gradient-inspired colors
    options.color_background = "#F5F3FF".to_string();  // Very light purple (cleaner)
    options.color_data = "#4d3695".to_string();        // Brand purple
    options.color_finder = "#5B34A8".to_string();      // Slightly lighter purple for contrast
    
    // Smooth, modern shapes
    options.shape_module = ModuleShape::Circle;        // Circular dots for premium look
    options.shape_finder = FinderShape::Rounded(2.0);  // Nicely rounded finders
    
    let logo_svg = fs::read_to_string("logo-icon.svg")
        .expect("Failed to read logo file");
    let logo_base64 = base64_encode_svg(&logo_svg);
    options.center_image_url = Some(logo_base64);
    options.overlay_scale = 0.28;  // Slightly smaller for better scannability
    
    let svg = qr.render_svg(&options);
    save_svg("output/gradient_logo.svg", &svg);
    
    // Style 5: Ultra Premium (white background, bold purple)
    println!("5. Generating ultra-premium style with logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Clean, professional design
    options.color_background = "#FFFFFF".to_string();  // Pure white
    options.color_data = "#4d3695".to_string();        // Brand purple
    options.color_finder = "#4d3695".to_string();      // Brand purple
    
    // Premium rounded squares
    options.shape_module = ModuleShape::RoundedSquare(0.35);
    options.shape_finder = FinderShape::Rounded(1.8);
    
    let logo_svg = fs::read_to_string("logo-icon.svg")
        .expect("Failed to read logo file");
    let logo_base64 = base64_encode_svg(&logo_svg);
    options.center_image_url = Some(logo_base64);
    options.overlay_scale = 0.26;
    
    let svg = qr.render_svg(&options);
    save_svg("output/premium.svg", &svg);
    
    // Style 6: Branded Finders (No center logo - finders ARE the branding!)
    println!("6. Generating QR with branded finder patterns...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Clean design focusing on the branded finder patterns
    options.color_background = "#FFFFFF".to_string();
    options.color_data = "#1a1a1a".to_string();         // Dark gray for subtle data
    options.color_finder = "#4d3695".to_string();       // Brand purple for prominent finders
    
    // Smooth, modern shapes
    options.shape_module = ModuleShape::RoundedSquare(0.25);
    options.shape_finder = FinderShape::Rounded(2.2);   // Heavily rounded
    
    // NO center overlay - the branded finders are the focal point!
    options.overlay_scale = 0.0;
    
    let svg = qr.render_svg(&options);
    save_svg("output/branded_finders.svg", &svg);
    
    // Style 7: Ultra Minimal with Branded Finders
    println!("7. Generating ultra-minimal with branded finders...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Maximum contrast, maximum brand visibility
    options.color_background = "#FFFFFF".to_string();   // Pure white
    options.color_data = "#000000".to_string();         // Pure black
    options.color_finder = "#4d3695".to_string();       // Brand purple stands out
    
    // Clean, professional
    options.shape_module = ModuleShape::Square;
    options.shape_finder = FinderShape::Rounded(1.5);
    
    let svg = qr.render_svg(&options);
    save_svg("output/finders_minimal.svg", &svg);
    
    // Style 8: Gradient-inspired with Branded Finders
    println!("8. Generating gradient style with branded finders...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Gradient look with prominent purple finders
    options.color_background = "#FAF5FF".to_string();   // Light purple
    options.color_data = "#6B4B8A".to_string();         // Medium purple
    options.color_finder = "#4d3695".to_string();       // Brand purple (darkest)
    
    // All rounded for cohesive premium look
    options.shape_module = ModuleShape::Circle;
    options.shape_finder = FinderShape::Rounded(2.5);
    
    let svg = qr.render_svg(&options);
    save_svg("output/gradient_finders.svg", &svg);
    
    // Style 9: Gradient Colors with Rectangular Modules (Gradient Minimal with Logo)
    println!("9. Generating gradient minimal with logo...");
    let qr = FancyQr::from_text(url).expect("Failed to create QR code");
    let mut options = FancyOptions::default();
    
    // Gradient color palette with rectangular modules
    options.color_background = "#FAF5FF".to_string();   // Light purple background
    options.color_data = "#6B4B8A".to_string();         // Medium purple data
    options.color_finder = "#4d3695".to_string();       // Brand purple finders
    
    // Rectangular modules for clean, scannable look
    options.shape_module = ModuleShape::Square;
    options.shape_finder = FinderShape::Rounded(1.5);   // Rounded finders for brand
    
    // Add logo to center
    let logo_svg = fs::read_to_string("logo-icon.svg")
        .expect("Failed to read logo file");
    let logo_base64 = base64_encode_svg(&logo_svg);
    options.center_image_url = Some(logo_base64);
    options.overlay_scale = 0.25;
    
    let svg = qr.render_svg(&options);
    save_svg("output/gradient_minimal_logo.svg", &svg);
    
    println!("\n✨ All branded QR codes generated in ./output/ directory!");
    println!("\n📦 With Center Logo:");
    println!("   - with_logo.svg");
    println!("   - with_logo_base64.svg");
    println!("   - minimal_logo.svg");
    println!("   - gradient_logo.svg");
    println!("   - premium.svg");
    println!("   - gradient_minimal_logo.svg (✨ Gradient colors + rectangular modules + logo)");
    println!("\n⚓ Branded Finder Patterns (No center logo - finders are the brand!):");
    println!("   - branded_finders.svg");
    println!("   - finders_minimal.svg");
    println!("   - gradient_finders.svg");
}

fn save_svg(filename: &str, svg: &str) {
    let mut file = File::create(filename)
        .expect("Failed to create file");
    file.write_all(svg.as_bytes())
        .expect("Failed to write to file");
    println!("   ✓ Saved: {}", filename);
}

// Simple base64 encoding for SVG data URI
fn base64_encode_svg(svg: &str) -> String {
    // Create a simple base64 encoder
    let mut encoded = String::new();
    let bytes = svg.as_bytes();
    
    // Use standard base64 encoding
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        
        encoded.push(ALPHABET[b1] as char);
        encoded.push(ALPHABET[b2] as char);
        encoded.push(if chunk.len() > 1 { ALPHABET[b3] as char } else { '=' });
        encoded.push(if chunk.len() > 2 { ALPHABET[b4] as char } else { '=' });
    }
    
    format!("data:image/svg+xml;base64,{}", encoded)
}

