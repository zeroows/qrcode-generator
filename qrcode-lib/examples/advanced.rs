// Example: Advanced QR code generation with segments
//
// This example demonstrates fine-grained control over QR code generation
// using segments and advanced encoding parameters.

use qrcode_lib::{QrCode, QrCodeEcc, QrSegment, Version, Mask};
use qrcode_lib::render::to_ascii_art;

fn main() {
    println!("=== Advanced QR Code Generation ===\n");
    
    // Example 1: Numeric mode for better efficiency
    println!("1. Numeric mode (more efficient for numbers):");
    let numbers = "314159265358979323846";
    let seg_numeric = QrSegment::make_numeric(numbers);
    let qr_numeric = QrCode::encode_segments(&[seg_numeric], QrCodeEcc::Low)
        .expect("Failed to create QR code");
    println!("   Numbers: {}", numbers);
    println!("   Size: {}x{}\n", qr_numeric.size(), qr_numeric.size());
    
    // Example 2: Alphanumeric mode
    println!("2. Alphanumeric mode:");
    let alphanum = "HELLO WORLD 123";
    let seg_alphanum = QrSegment::make_alphanumeric(alphanum);
    let qr_alphanum = QrCode::encode_segments(&[seg_alphanum], QrCodeEcc::Medium)
        .expect("Failed to create QR code");
    println!("   Text: {}", alphanum);
    println!("   Size: {}x{}\n", qr_alphanum.size(), qr_alphanum.size());
    
    // Example 3: Binary data
    println!("3. Binary data:");
    let binary_data = b"Hello, \xF0\x9F\x92\x96 World!";
    let qr_binary = QrCode::encode_binary(binary_data, QrCodeEcc::Quartile)
        .expect("Failed to create QR code");
    println!("   Data: {} bytes", binary_data.len());
    println!("   Size: {}x{}\n", qr_binary.size(), qr_binary.size());
    
    // Example 4: Advanced encoding with specific version and mask
    println!("4. Advanced encoding with specific parameters:");
    let text = "https://example.com";
    let segs = QrSegment::make_segments(text);
    let qr_advanced = QrCode::encode_segments_advanced(
        &segs,
        QrCodeEcc::High,         // High error correction
        Version::new(5),         // Minimum version 5
        Version::new(10),        // Maximum version 10
        Some(Mask::new(3)),      // Use mask pattern 3
        false                    // Don't boost ECC
    ).expect("Failed to create QR code");
    println!("   URL: {}", text);
    println!("   Version: {:?}", qr_advanced.version());
    println!("   Size: {}x{}", qr_advanced.size(), qr_advanced.size());
    println!("   Mask: {:?}", qr_advanced.mask());
    println!("   ECC: {:?}\n", qr_advanced.error_correction_level());
    
    // Display one of them
    println!("ASCII representation of example 4:");
    let ascii = to_ascii_art(&qr_advanced, 1);
    println!("{}", ascii);
}

